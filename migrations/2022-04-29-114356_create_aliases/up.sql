-- Your SQL goes here
create table Aliases (
  alias text primary key,
  filepath text not null,
  foreign key (filepath) references Nodes(filepath) on delete cascade
)
