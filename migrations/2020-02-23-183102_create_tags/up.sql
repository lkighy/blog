-- Your SQL goes here
create table tags(
    id serial primary key,
    tag varchar not null unique,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
)