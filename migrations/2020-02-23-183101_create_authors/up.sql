-- Your SQL goes here
CREATE TABLE authors(
    id serial primary key,
    author varchar not null,
    email varchar not null,
    birth date default current_date not null,
    introduce text default '',
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
)