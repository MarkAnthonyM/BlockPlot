CREATE TABLE skillblocks (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    category VARCHAR NOT NULL,
    offline_category BOOLEAN NOT NULL DEFAULT 'f',
    skill_name TEXT NOT NULL,
    skill_description TEXT NOT NULL
)