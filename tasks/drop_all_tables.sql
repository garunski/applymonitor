-- Drop all tables to reset the database
-- This script is used by db:reset-local and db:reset-prod tasks

DROP TABLE IF EXISTS email_scans;
DROP TABLE IF EXISTS gmail_tokens;
DROP TABLE IF EXISTS password_reset_tokens;
DROP TABLE IF EXISTS user_credentials;
DROP TABLE IF EXISTS user_providers;
DROP TABLE IF EXISTS jobs;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS d1_migrations;

