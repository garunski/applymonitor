CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  provider TEXT NOT NULL,
  provider_sub TEXT NOT NULL,
  email TEXT,
  name TEXT,
  avatar TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  last_login DATETIME
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_provider_sub ON users(provider, provider_sub);

