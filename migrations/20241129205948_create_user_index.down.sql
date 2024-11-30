-- Add down migration script here
DROP INDEX IF EXISTS idx_name;
DROP INDEX IF EXISTS idx_name_desc;
DROP INDEX IF EXISTS idx_email;
DROP INDEX IF EXISTS idx_email_desc;