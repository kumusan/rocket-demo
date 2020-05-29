table! {
    pub todos (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        done -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    todos,
);
