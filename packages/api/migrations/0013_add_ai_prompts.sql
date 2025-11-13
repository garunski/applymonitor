-- Create ai_prompts table for storing editable AI prompts
CREATE TABLE IF NOT EXISTS ai_prompts (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  stage TEXT NOT NULL, -- 'classify', 'extract', 'summarize'
  prompt TEXT NOT NULL,
  is_active BOOLEAN DEFAULT false,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Index for finding active prompts by stage
CREATE INDEX IF NOT EXISTS idx_ai_prompts_stage_active ON ai_prompts(stage, is_active);

-- Seed initial prompts for all 3 stages
INSERT INTO ai_prompts (id, name, stage, prompt, is_active) VALUES
('classify-v1', 'Classification v1', 'classify', 'You categorize job search emails.

Email from: {{from_email}}
Subject: {{subject}}
Body: {{body}}

Return JSON only:
{
  "category": "interview|rejection|new_job|application_sent|other",
  "confidence": 0.0-1.0
}

Rules:
- Interview invites/confirmations → "interview"
- "Unfortunately" or "not moving forward" → "rejection"  
- New opportunities being shared → "new_job"
- Application received confirmations → "application_sent"
- If unsure → "other" with confidence < 0.6', true),

('extract-v1', 'Extraction v1', 'extract', 'Extract information from this {{category}} email.

Email:
From: {{from_email}}
Subject: {{subject}}
Body: {{body}}

Return JSON only:
{
  "company": "company name or null",
  "job_title": "title or null",
  "recruiter_name": "name or null",
  "recruiter_email": "email or null",
  "interview_date": "ISO8601 or null",
  "location": "string or null",
  "remote": true/false/null
}

Only include fields you find. Use null if missing.', true),

('summarize-v1', 'Summarization v1', 'summarize', 'Summarize this {{category}} email in one sentence.

Email subject: {{subject}}
From: {{from_email}}
Body: {{body}}

Return JSON only:
{
  "summary": "one sentence here"
}

Focus on what the user needs to know and do.', true);

