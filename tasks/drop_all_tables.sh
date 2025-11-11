#!/bin/bash
# Auto-discover and drop all tables from the database
# This script queries sqlite_master to find all tables and drops them

ENV_FLAG="$1"  # --env local or --env production
REMOTE_FLAG="$2"  # --remote

# Query for all table names (excluding sqlite system tables)
TABLES=$(wrangler d1 execute DB $ENV_FLAG $REMOTE_FLAG --command "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE '_cf_%';" 2>/dev/null | \
  grep -o '"name": "[^"]*"' | \
  sed 's/"name": "\(.*\)"/\1/' | \
  tr '\n' ' ')

if [ -z "$TABLES" ]; then
  echo "No tables found to drop."
  exit 0
fi

# Build DROP TABLE statements
DROP_STATEMENTS=""
for table in $TABLES; do
  DROP_STATEMENTS="${DROP_STATEMENTS}DROP TABLE IF EXISTS ${table}; "
done

# Execute the DROP statements
echo "Dropping tables: $TABLES"
wrangler d1 execute DB $ENV_FLAG $REMOTE_FLAG --command "$DROP_STATEMENTS" || echo "Error dropping tables"

