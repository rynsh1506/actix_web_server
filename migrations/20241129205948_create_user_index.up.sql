-- Add up migration script here
CREATE INDEX idx_name ON users (name);
CREATE INDEX idx_name_desc ON users (name DESC);
CREATE UNIQUE INDEX idx_email ON users (email);
CREATE UNIQUE INDEX idx_email_desc ON users (email DESC);