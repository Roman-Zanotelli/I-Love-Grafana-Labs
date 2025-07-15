CREATE TABLE IF NOT EXISTS auth (
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    account_id UUID NOT NULL,
    PRIMARY KEY (email, username)
);