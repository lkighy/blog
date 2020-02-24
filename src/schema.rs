table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        md_body -> Nullable<Text>,
        html_body -> Nullable<Text>,
        book_id -> Nullable<Int4>,
        tags -> Nullable<Array<Text>>,
        created_time -> Nullable<Timestamp>,
        updated_time -> Nullable<Timestamp>,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    authors (id) {
        id -> Int4,
        author -> Varchar,
        email -> Varchar,
        birth -> Nullable<Date>,
        introduce -> Nullable<Text>,
        created_time -> Nullable<Timestamp>,
        updated_time -> Nullable<Timestamp>,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    books (id) {
        id -> Int4,
        book_name -> Varchar,
        description -> Nullable<Varchar>,
        tags -> Nullable<Array<Text>>,
        created_time -> Nullable<Timestamp>,
        updated_time -> Nullable<Timestamp>,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    catalogs (id) {
        id -> Int4,
        book_id -> Nullable<Int4>,
        rank -> Int4,
        grade -> Int4,
        parent_id -> Nullable<Int4>,
        created_time -> Nullable<Timestamp>,
        updated_time -> Nullable<Timestamp>,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    child_comments (id) {
        id -> Int4,
        email -> Varchar,
        user_name -> Varchar,
        content -> Text,
        md_content -> Text,
        parent_id -> Int4,
        child_comment_id -> Nullable<Int4>,
        ip -> Varchar,
        created_time -> Nullable<Timestamp>,
        updated_time -> Nullable<Timestamp>,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    comments (id) {
        id -> Int4,
        email -> Varchar,
        user_name -> Varchar,
        content -> Text,
        md_content -> Text,
        ip -> Varchar,
        created_time -> Nullable<Timestamp>,
        updated_time -> Nullable<Timestamp>,
        deleted_time -> Nullable<Timestamp>,
    }
}

table! {
    tags (id) {
        id -> Int4,
        tag -> Varchar,
        created_time -> Nullable<Timestamp>,
        updated_time -> Nullable<Timestamp>,
        deleted_time -> Nullable<Timestamp>,
    }
}

joinable!(articles -> books (book_id));
joinable!(catalogs -> books (book_id));
joinable!(child_comments -> comments (parent_id));

allow_tables_to_appear_in_same_query!(
    articles,
    authors,
    books,
    catalogs,
    child_comments,
    comments,
    tags,
);
