CREATE TABLE transfers (
                           id SERIAL PRIMARY KEY,
                           from_address TEXT NOT NULL,
                           to_address TEXT NOT NULL,
                           amount BIGINT NOT NULL,
                           timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);