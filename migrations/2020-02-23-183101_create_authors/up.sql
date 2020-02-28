-- Your SQL goes here
CREATE TABLE authors(
    id serial primary key,
    author varchar not null,
    email varchar not null,
    hobby varchar not null default '',
    birth date default current_date not null,
    introduce text default '',
    lines jsonb not null default '{}'::jsonb,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
)