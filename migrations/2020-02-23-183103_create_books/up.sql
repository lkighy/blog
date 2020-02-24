-- Your SQL goes here
create table books(
    id serial primary key,
    book_name varchar not null,
    description varchar default '',
    tags text[],
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
)