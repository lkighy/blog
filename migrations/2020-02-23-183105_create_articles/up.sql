-- Your SQL goes here
create table articles (
    id serial primary key,
    title varchar not null,
    description varchar default '',
    md_body text default '',
    html_body text default '',
    book_id integer references books(id),
    tags text[],
    parent_id integer references catalogs(id),
    created_at timestamp default current_timestamp not null,
    updated_at timestamp default current_timestamp not null
)