CREATE DATABASE ledger_db;
CREATE USER api_user WITH ENCRYPTED PASSWORD 'super-secure-password';
GRANT ALL PRIVILEGES ON DATABASE ledger_db TO api_user;