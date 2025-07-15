CREATE DATABASE auth_db;
CREATE TABLE IF NOT EXISTS auth (
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    account_id UUID NOT NULL,
    PRIMARY KEY (email, username)
);

CREATE USER api_user WITH ENCRYPTED PASSWORD 'super-secure-password';
GRANT ALL PRIVILEGES ON DATABASE auth_db TO api_user;