-- Add AI processing fields to emails table
ALTER TABLE emails ADD COLUMN ai_processed BOOLEAN DEFAULT false;
ALTER TABLE emails ADD COLUMN needs_review BOOLEAN DEFAULT false;

-- Index for finding unprocessed emails
CREATE INDEX IF NOT EXISTS idx_emails_ai_processed ON emails(ai_processed);
CREATE INDEX IF NOT EXISTS idx_emails_needs_review ON emails(needs_review);

