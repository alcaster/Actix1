CREATE TABLE games
(
    id SERIAL PRIMARY KEY,
    user_id VARCHAR NOT NULL references users(id)
);