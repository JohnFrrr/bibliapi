// @generated automatically by Diesel CLI.

diesel::table! {
    BOOK (ID) {
        ID -> Integer,
        NAME -> Text,
    }
}

diesel::table! {
    CHAPTER (ID) {
        ID -> Integer,
        BOOK_ID -> Integer,
        NUMBER -> Integer,
    }
}

diesel::table! {
    VERSE (ID) {
        ID -> Integer,
        CHAPTER_ID -> Integer,
        NUMBER -> Integer,
        CONTENT -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::joinable!(CHAPTER -> BOOK (BOOK_ID));
diesel::joinable!(VERSE -> CHAPTER (CHAPTER_ID));

diesel::allow_tables_to_appear_in_same_query!(
    BOOK,
    CHAPTER,
    VERSE,
    posts,
);
