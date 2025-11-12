-- Create email_contacts table for storing user-specific contact information
CREATE TABLE IF NOT EXISTS email_contacts (
  email TEXT NOT NULL,
  user_id TEXT NOT NULL,
  name TEXT,
  linkedin TEXT,
  website TEXT,
  is_system INTEGER DEFAULT 0,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (email, user_id),
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_email_contacts_user_id ON email_contacts(user_id);
CREATE INDEX IF NOT EXISTS idx_email_contacts_email ON email_contacts(email);

