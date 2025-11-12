-- Create job_comments table for storing comments on jobs
CREATE TABLE IF NOT EXISTS job_comments (
  id TEXT PRIMARY KEY,
  job_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  content TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE CASCADE,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_job_comments_job_id ON job_comments(job_id);
CREATE INDEX IF NOT EXISTS idx_job_comments_created_at ON job_comments(created_at);

