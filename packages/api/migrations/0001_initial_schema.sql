-- Initial database schema with UUID-based user IDs
-- This migration creates all tables with UUID (TEXT) primary keys

-- Users table (provider-agnostic) with UUID primary key
CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY,
  email TEXT UNIQUE,
  name TEXT,
  picture TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- User providers table (one-to-many relationship)
CREATE TABLE IF NOT EXISTS user_providers (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  provider_id TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  UNIQUE(provider, provider_id)
);

-- User credentials table (for local auth only)
CREATE TABLE IF NOT EXISTS user_credentials (
  user_id TEXT PRIMARY KEY,
  password_hash TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Password reset tokens table
CREATE TABLE IF NOT EXISTS password_reset_tokens (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  token_hash TEXT NOT NULL,
  expires_at DATETIME NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Jobs table
CREATE TABLE IF NOT EXISTS jobs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  company TEXT NOT NULL,
  location TEXT,
  status TEXT NOT NULL DEFAULT 'open',
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_user_providers_provider_id ON user_providers(provider, provider_id);
CREATE INDEX IF NOT EXISTS idx_user_providers_user_id ON user_providers(user_id);
CREATE INDEX IF NOT EXISTS idx_password_reset_tokens_token_hash ON password_reset_tokens(token_hash);
CREATE INDEX IF NOT EXISTS idx_password_reset_tokens_expires_at ON password_reset_tokens(expires_at);

-- Seed data
INSERT INTO jobs (title, company, location, status) VALUES
  ('Senior Software Engineer', 'Tech Corp', 'San Francisco, CA', 'open'),
  ('Product Manager', 'StartupXYZ', 'Remote', 'open'),
  ('DevOps Engineer', 'Cloud Systems', 'New York, NY', 'applied'),
  ('Frontend Developer', 'Web Solutions', 'Austin, TX', 'open'),
  ('Backend Developer', 'API Masters', 'Seattle, WA', 'interview'),
  ('Full Stack Engineer', 'Digital Innovations', 'Remote', 'open'),
  ('Data Engineer', 'Analytics Pro', 'Boston, MA', 'rejected'),
  ('Mobile Developer', 'App Creators', 'Los Angeles, CA', 'open');

