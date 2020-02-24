-- Your SQL goes here
create table books(
    id serial primary key,
    book_name varchar not null,
    description varchar default '',
    tags text[],
    created_time timestamp default current_timestamp,
    updated_time timestamp default current_timestamp,
    deleted_time timestamp
)