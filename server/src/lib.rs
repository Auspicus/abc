#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

use diesel::sqlite::SqliteConnection;
use models::{Experiment, Session};

pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn create_db_pool(database_url: String) -> DbPool {
    r2d2::Pool::builder()
        .build(ConnectionManager::new(database_url))
        .expect("Failed to create database connection pool.")
}

pub fn get_experiment(conn: &SqliteConnection, eid: String) -> Option<Experiment> {
    use self::schema::experiments::dsl::*;

    let results = experiments
        .filter(id.eq(&eid))
        .limit(1)
        .load::<Experiment>(conn)
        .unwrap_or_default();

    results.into_iter().next()
}

pub fn get_experiments(conn: &SqliteConnection, limit: i64) -> Vec<Experiment> {
    use self::schema::experiments::dsl::*;

    experiments
        .limit(limit)
        .load::<Experiment>(conn)
        .unwrap_or_default()
}

pub fn create_experiment(conn: &SqliteConnection, experiment: &Experiment) -> Experiment {
    diesel::insert_into(schema::experiments::table)
        .values(experiment)
        .execute(conn)
        .expect("Failed to create new session.");

    experiment.clone()
}

pub fn get_session(conn: &SqliteConnection, sid: &str, eid: &str) -> Option<Session> {
    use self::schema::sessions::dsl::*;

    let results = sessions
        .filter(id.eq(sid))
        .filter(experiment_id.eq(eid))
        .limit(1)
        .load::<Session>(conn)
        .unwrap_or_default();

    results.into_iter().next()
}

pub fn get_sessions(conn: &SqliteConnection, limit: i64) -> Vec<Session> {
    use self::schema::sessions::dsl::*;

    sessions
        .limit(limit)
        .load::<Session>(conn)
        .unwrap_or_default()
}

pub fn create_session(conn: &SqliteConnection, experiment_id: &str, variant: i32) -> Session {
    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_session = Session {
        id: uuid,
        experiment_id: experiment_id.to_owned(),
        variant,
    };

    diesel::insert_into(schema::sessions::table)
        .values(&new_session)
        .execute(conn)
        .expect("Failed to create new session.");

    new_session
}
