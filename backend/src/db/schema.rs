table! {
    skillblocks (block_id) {
        block_id -> Int4,
        user_id -> Nullable<Int4>,
        category -> Varchar,
        offline_category -> Nullable<Bool>,
        skill_name -> Varchar,
        skill_description -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        auth_id -> Varchar,
        api_key -> Nullable<Varchar>,
        key_present -> Nullable<Bool>,
        block_count -> Int4,
    }
}

joinable!(skillblocks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    skillblocks,
    users,
);
