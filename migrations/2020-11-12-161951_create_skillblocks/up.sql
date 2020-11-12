CREATE TABLE skillblocks (
    id SERIAL PRIMARY KEY,
    category VARCHAR NOT NULL,
    skill_description TEXT NOT NULL,
    skill_name TEXT NOT NULL,
)