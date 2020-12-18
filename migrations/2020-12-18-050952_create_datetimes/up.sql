CREATE TABLE date_times (
    id SERIAL PRIMARY KEY,
    block_id INT,
    day_date VARCHAR,
    day_time INT,
    CONSTRAINT fk_skillblocks
        FOREIGN KEY(block_id)
            REFERENCES skillblocks(block_id)
            ON DELETE CASCADE
)