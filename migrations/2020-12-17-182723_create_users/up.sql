CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    auth_id VARCHAR NOT NULL,
    api_key VARCHAR,
    key_present BOOLEAN DEFAULT 'f',
    block_count INT NOT NULL
)