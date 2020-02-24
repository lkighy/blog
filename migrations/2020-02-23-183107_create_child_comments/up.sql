-- Your SQL goes here
create table child_comments(
    id serial primary key,
    email varchar not null,
    user_name varchar not null,
    content text not null,
    md_content text not null,
    parent_id integer references comments(id) not null,
    child_comment_id integer references child_comments(id),
    ip varchar not null,
    created_time timestamp default current_timestamp,
    updated_time timestamp default current_timestamp,
    deleted_time timestamp
)