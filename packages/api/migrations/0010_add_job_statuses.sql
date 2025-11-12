-- Create job_statuses table
CREATE TABLE IF NOT EXISTS job_statuses (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  display_name TEXT NOT NULL,
  description TEXT
);

-- Seed initial statuses (IDs increment by 100 to allow inserting between)
INSERT OR IGNORE INTO job_statuses (id, name, display_name, description) VALUES
(100, 'open', 'Open', 'Job saved, not yet applied'),
(200, 'applied', 'Applied', 'Application submitted'),
(300, 'interviewing', 'Interviewing', 'In interview process'),
(400, 'offer', 'Offer', 'Offer received'),
(500, 'rejected', 'Rejected', 'Application rejected');

-- Add status_id column to jobs table
ALTER TABLE jobs ADD COLUMN status_id INTEGER;

-- Set default status_id for existing jobs based on status text
UPDATE jobs SET status_id = 100 WHERE status = 'open' AND status_id IS NULL;
UPDATE jobs SET status_id = 200 WHERE status = 'applied' AND status_id IS NULL;
UPDATE jobs SET status_id = 300 WHERE status = 'interviewing' AND status_id IS NULL;
UPDATE jobs SET status_id = 400 WHERE status = 'offer' AND status_id IS NULL;
UPDATE jobs SET status_id = 500 WHERE status = 'rejected' AND status_id IS NULL;

-- Set default status_id for any remaining NULL values (shouldn't happen, but safety)
UPDATE jobs SET status_id = 100 WHERE status_id IS NULL;

-- Add foreign key constraint
-- Note: SQLite doesn't enforce foreign keys by default, but we add it for documentation
-- and potential future use with PRAGMA foreign_keys = ON

-- Create index for faster lookups
CREATE INDEX IF NOT EXISTS idx_jobs_status_id ON jobs(status_id);

