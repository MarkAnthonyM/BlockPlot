table! {
    users (user_id) {
        user_id -> Int4,
        auth_id -> Varchar,
        api_key -> Nullable<Varchar>,
        key_present -> Nullable<Bool>,
        block_count -> Int4,
    }
}
