extern crate diesel;

diesel::table! {
    pub todos (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        done -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
);
