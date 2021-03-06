table! {
    date_times (id) {
        id -> Int4,
        block_id -> Nullable<Int4>,
        day_time -> Int4,
        day_date -> Timestamp,
    }
}

table! {
    skillblocks (block_id) {
        block_id -> Int4,
        user_id -> Nullable<Int4>,
        category -> Varchar,
        offline_category -> Bool,
        skill_name -> Varchar,
        skill_description -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        auth_id -> Varchar,
        api_key -> Nullable<Varchar>,
        key_present -> Bool,
        block_count -> Int4,
        created_at -> Timestamp,
        blocks_last_fetched -> Timestamp,
        last_login -> Timestamp,
    }
}

joinable!(date_times -> skillblocks (block_id));
joinable!(skillblocks -> users (user_id));

allow_tables_to_appear_in_same_query!(date_times, skillblocks, users,);
