create table cards (
  id text not null primary key,
  list_id text not null,
  title text not null,
  body text not null,
  created_at text not null,
  updated_at text not null
);

