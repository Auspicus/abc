-- Your SQL goes here
CREATE TABLE experiments (
  id TEXT PRIMARY KEY NOT NULL,
  variants INTEGER NOT NULL
);

INSERT
  INTO experiments (id, variants)
  VALUES("experiment-1", 2)