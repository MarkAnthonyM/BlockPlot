table! {
    skillblocks (id) {
        id -> Int4,
        username -> Text,
        category -> Varchar,
        offline_category -> Bool,
        skill_name -> Text,
        skill_description -> Text,
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
    skillblocks,
    users,
);
