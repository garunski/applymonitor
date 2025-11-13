-- Create ai_daily_stats table for daily rollup metrics
CREATE TABLE IF NOT EXISTS ai_daily_stats (
  date DATE PRIMARY KEY,
  emails_processed INTEGER DEFAULT 0,
  avg_confidence REAL,
  needs_review_count INTEGER DEFAULT 0,
  category_breakdown TEXT -- JSON string with category distribution
);

