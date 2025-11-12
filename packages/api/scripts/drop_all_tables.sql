-- Drop all tables to reset the database
-- Used by db:reset-local and db:reset-prod tasks
-- Order matters: drop child tables (with foreign keys) before parent tables

-- Drop migration tracking first
DROP TABLE IF EXISTS d1_migrations;

-- Drop tables with foreign keys (child tables) first
DROP TABLE IF EXISTS job_comments;
DROP TABLE IF EXISTS emails;
DROP TABLE IF EXISTS email_contacts;
DROP TABLE IF EXISTS email_scans;
DROP TABLE IF EXISTS gmail_tokens;
DROP TABLE IF EXISTS password_reset_tokens;
DROP TABLE IF EXISTS user_credentials;
DROP TABLE IF EXISTS user_providers;

-- Drop standalone tables
DROP TABLE IF EXISTS system_email_domains;

-- Drop parent tables (referenced by others) last
DROP TABLE IF EXISTS jobs;
DROP TABLE IF EXISTS users;

