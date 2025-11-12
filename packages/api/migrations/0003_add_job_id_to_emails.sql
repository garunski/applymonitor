-- Add job_id column to emails table to link emails to jobs
ALTER TABLE emails ADD COLUMN job_id TEXT;
CREATE INDEX IF NOT EXISTS idx_emails_job_id ON emails(job_id);
-- Note: Foreign key constraint will be added after ensuring all existing data is valid
-- ALTER TABLE emails ADD FOREIGN KEY (job_id) REFERENCES jobs(id) ON DELETE SET NULL;

