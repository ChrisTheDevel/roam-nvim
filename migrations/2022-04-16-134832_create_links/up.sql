-- Your SQL goes here
CREATE TABLE Links (
  id INT PRIMARY KEY NOT NULL,
  source TEXT NOT NULL,
  destination TEXT NOT NULL,
  linktype TEXT NOT NULL,
  FOREIGN KEY (source) REFERENCES files(filepath) ON DELETE CASCADE,
  FOREIGN KEY (destination) REFERENCES files(filepath) ON DELETE CASCADE
)
