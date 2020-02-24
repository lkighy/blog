-- Your SQL goes here
create table comments(
    id serial primary key,
    email varchar not null ,
    user_name varchar not null,
    content text not null,
    md_content text not null,
    ip varchar not null,
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
)