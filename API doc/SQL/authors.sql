CREATE TABLE authors(
    id serial primary key,
    author varchar not null,
    email varchar not null,
    birth date default current_date,
    introduce text default '',
    created_time timestamp default current_timestamp,
    updated_time timestamp default current_timestamp,
    deleted_time timestamp,
);