table! {
    date_times (id) {
        id -> Int4,
    }
}

table! {
    users (auth_user) {
        auth_user -> Text,
        api_key -> Nullable<Text>,
        key_stored -> Bool,
        block_count -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    date_times,
    users,
);
