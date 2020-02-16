CREATE TABLE games
(
    id     SERIAL PRIMARY KEY,
    start  TIMESTAMP NOT NULL,
    active BOOLEAN   NOT NULL
);