-- Your SQL goes here
create table catalogs(
    id serial primary key,
    book_id integer references books(id),
    rank integer not null,
    grade integer not null,
    parent_id integer references catalogs(id),
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
)