table! {
    games (id) {
        id -> Int4,
        start -> Timestamp,
        active -> Bool,
    }
}

table! {
    user_games (id) {
        id -> Int4,
        user_id -> Varchar,
        game_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

joinable!(user_games -> games (game_id));
joinable!(user_games -> users (user_id));

allow_tables_to_appear_in_same_query!(
    games,
    user_games,
    users,
);
