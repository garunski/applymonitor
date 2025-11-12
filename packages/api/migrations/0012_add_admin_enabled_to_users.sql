-- Add admin and enabled columns to users table
ALTER TABLE users ADD COLUMN is_admin BOOLEAN DEFAULT 0;
ALTER TABLE users ADD COLUMN enabled BOOLEAN DEFAULT 1;

-- Create index on is_admin for query performance
CREATE INDEX IF NOT EXISTS idx_users_is_admin ON users(is_admin);

