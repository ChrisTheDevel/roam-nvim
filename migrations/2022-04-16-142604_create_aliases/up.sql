-- Your SQL goes here
create table Aliases(
  alias TEXT PRIMARY KEY NOT NULL,
  filepath TEXT NOT NULL,
  foreign key (filepath) references Files(filepath)
)
