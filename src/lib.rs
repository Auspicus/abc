#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
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

pub fn get_session(conn: &SqliteConnection, sid: String) -> Option<Session> {
  use self::schema::sessions::dsl::*;

  let results = sessions.filter(id.eq(&sid))
    .limit(1)
    .load::<Session>(conn)
    .unwrap_or(vec![]);

  return results.into_iter().nth(0)
}

pub fn create_session(conn: &SqliteConnection, variant: u8) -> Session {
    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_session = Session {
        id: uuid,
        variant: i32::from(variant),
    };

    diesel::insert_into(schema::sessions::table)
        .values(&new_session)
        .execute(conn)
        .expect("Failed to create new session.");
    
    return new_session
}
