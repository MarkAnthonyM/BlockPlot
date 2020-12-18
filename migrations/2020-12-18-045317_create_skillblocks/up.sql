CREATE TABLE skillblocks (
    block_id SERIAL PRIMARY KEY,
    user_id INT,
    category VARCHAR NOT NULL,
    offline_category BOOLEAN DEFAULT 'f',
    skill_name VARCHAR NOT NULL,
    skill_description VARCHAR NOT NULL,
    CONSTRAINT fk_users
        FOREIGN KEY(user_id)
            REFERENCES users(user_id)
            ON DELETE CASCADE
)