table! {
    games (id) {
        id -> Int4,
        user_id -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

joinable!(games -> users (user_id));

allow_tables_to_appear_in_same_query!(
    games,
    users,
);
