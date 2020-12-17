CREATE TABLE users (
    auth_user TEXT PRIMARY KEY,
    api_key TEXT,
    key_stored BOOLEAN NOT NULL DEFAULT 'f',
    block_count INTEGER NOT NULL
)