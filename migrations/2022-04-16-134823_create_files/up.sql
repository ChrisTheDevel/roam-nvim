-- Your SQL goes here
CREATE TABLE Files (
  filepath TEXT PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  atime INT NOT NULL,
  mtime INT NOT NULL
)
