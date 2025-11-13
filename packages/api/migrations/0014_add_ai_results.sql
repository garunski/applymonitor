-- Create ai_results table for storing AI processing outputs
CREATE TABLE IF NOT EXISTS ai_results (
  id TEXT PRIMARY KEY,
  email_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  category TEXT, -- 'interview', 'rejection', 'new_job', 'application_sent', 'other'
  confidence REAL,
  company TEXT,
  job_title TEXT,
  summary TEXT,
  extracted_data TEXT, -- JSON string with everything else (dates, recruiters, etc)
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(email_id) REFERENCES emails(gmail_id) ON DELETE CASCADE,
  FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_ai_results_email_id ON ai_results(email_id);
CREATE INDEX IF NOT EXISTS idx_ai_results_user_id ON ai_results(user_id);
CREATE INDEX IF NOT EXISTS idx_ai_results_category ON ai_results(category);
CREATE INDEX IF NOT EXISTS idx_ai_results_created_at ON ai_results(created_at);

