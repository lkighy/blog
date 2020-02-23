create table articles (
    id serial primary key,
    title varchar not null,
    description varvhar default '',
    md_body text default '',
    html_body text default '',
    book_id integer references book(id),
    tags_id integer[] references tags(id),
    created_time timestamp default current_timestamp,
    updated_time timestamp default current_timestamp,
    deleted_time timestamp,
)