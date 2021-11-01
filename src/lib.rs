#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use models::Experiment;
use std::env;

use self::models::{Session};

pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn create_db_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    r2d2::Pool::builder()
      .build(ConnectionManager::new(database_url))
      .expect("Failed to create database connection pool.")
}

pub fn get_experiment(conn: &SqliteConnection, eid: String) -> Option<Experiment> {
  use self::schema::experiments::dsl::*;

  let results = experiments.filter(id.eq(&eid))
    .limit(1)
    .load::<Experiment>(conn)
    .unwrap_or(vec![]);

  return results.into_iter().nth(0)
}

pub fn create_experiment(conn: &SqliteConnection, experiment: &Experiment) -> Experiment {
  diesel::insert_into(schema::experiments::table)
    .values(experiment)
    .execute(conn)
    .expect("Failed to create new session.");

  return experiment.clone()
}

pub fn get_session(conn: &SqliteConnection, sid: &String, eid: &String) -> Option<Session> {
  use self::schema::sessions::dsl::*;

  let results = sessions
    .filter(id.eq(sid))
    .filter(experiment_id.eq(eid))
    .limit(1)
    .load::<Session>(conn)
    .unwrap_or(vec![]);

  return results.into_iter().nth(0)
}

pub fn create_session(conn: &SqliteConnection, experiment_id: &String, variant: i32) -> Session {
    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_session = Session {
        id: uuid,
        experiment_id: experiment_id.to_owned(),
        variant: i32::from(variant),
    };

    diesel::insert_into(schema::sessions::table)
        .values(&new_session)
        .execute(conn)
        .expect("Failed to create new session.");
    
    return new_session
}
