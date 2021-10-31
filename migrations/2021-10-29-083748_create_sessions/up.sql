-- Your SQL goes here
CREATE TABLE sessions (
  id TEXT PRIMARY KEY NOT NULL,
  experiment_id TEXT NOT NULL,
  variant INTEGER NOT NULL
)