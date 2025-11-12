-- Add emails table for storing scanned Gmail messages
-- Uses Gmail ID as primary key since Gmail IDs are globally unique
CREATE TABLE IF NOT EXISTS emails (
  gmail_id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  scan_id TEXT,
  thread_id TEXT NOT NULL,
  subject TEXT,
  "from" TEXT,
  "to" TEXT,
  snippet TEXT,
  date DATETIME,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY (scan_id) REFERENCES email_scans(id) ON DELETE SET NULL
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_emails_user_id ON emails(user_id);
CREATE INDEX IF NOT EXISTS idx_emails_scan_id ON emails(scan_id);
CREATE INDEX IF NOT EXISTS idx_emails_date ON emails(date);

