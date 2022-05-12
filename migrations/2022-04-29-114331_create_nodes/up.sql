-- Your SQL goes here
create table Nodes (
  filepath text primary key not null,
  title text not null,
  hash text not null,
  atime int not null,
  mtime int not null
)
