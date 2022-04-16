-- Your SQL goes here
create table Aliases(
  alias TEXT PRIMARY KEY,
  filepath TEXT,
  foreign key (filepath) references Files(filepath)
)
