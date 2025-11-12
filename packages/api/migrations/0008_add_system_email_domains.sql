-- Create system_email_domains table for configurable system email detection
CREATE TABLE IF NOT EXISTS system_email_domains (
  id TEXT PRIMARY KEY,
  domain_pattern TEXT NOT NULL UNIQUE,
  name TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Index for efficient lookups
CREATE INDEX IF NOT EXISTS idx_system_email_domains_pattern ON system_email_domains(domain_pattern);

