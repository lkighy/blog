create table catalogs(
    id serial primary key,
    book_id integer references book(id),
    rank integer not null,
    grade integer not null,
    parent_id integer references catalogs(id),
    created_time timestamp default current_timestamp,
    updated_time timestamp default current_timestamp,
    deleted_time timestamp,
)