create table books(
    id serial primary key,
    book_name varchar not null,
    description varchar default '',
    tags_id integer[],
    created_time timestamp default current_timestamp,
    updated_time timestamp default current_timestamp,
    deleted_time timestamp,
)