-- Add cc and bcc columns to emails table
ALTER TABLE emails ADD COLUMN cc TEXT;
ALTER TABLE emails ADD COLUMN bcc TEXT;

