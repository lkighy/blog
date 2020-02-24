create table articles (
    id serial primary key,
    title varchar not null,
    description varchar default '',
    md_body text default '',
    html_body text default '',
    book_id integer references books(id),
    tags_id text[],
    created_time timestamp default current_timestamp,
    updated_time timestamp default current_timestamp,
    deleted_time timestamp
)