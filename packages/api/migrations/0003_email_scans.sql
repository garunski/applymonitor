-- Email scans table
CREATE TABLE IF NOT EXISTS email_scans (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id TEXT NOT NULL,
  start_date DATETIME NOT NULL,
  end_date DATETIME NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',
  emails_found INTEGER DEFAULT 0,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  completed_at DATETIME,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_email_scans_user_id ON email_scans(user_id);
CREATE INDEX IF NOT EXISTS idx_email_scans_status ON email_scans(status);
CREATE INDEX IF NOT EXISTS idx_email_scans_created_at ON email_scans(created_at);

