CREATE TABLE user_games
(
    id      SERIAL PRIMARY KEY,
    user_id VARCHAR NOT NULL references users (id),
    game_id SERIAL  NOT NULL references games (id)
);