create table tags(
    id serial primary key,
    tag varchar not null unique,
    created_time timestamp default current_timestamp,
    updated_time timestamp default current_timestamp,
    deleted_time timestamp,
)