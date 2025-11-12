

# 1) High-level architecture (components & responsibilities)

* **Frontend (Dioxus)**

  * Web (WASM), Desktop (native), Mobile (shared components)
  * Real-time UI via WebSocket / SSE for updates (job created, email scanned, reminder fired)
  * Local caching + optimistic UI for quick responsiveness

* **API Gateway (Cloudflare Worker)**

  * Auth endpoints (OIDC, local email/password)
  * Job CRUD, comments, contacts
  * Gmail sync orchestrator (kick off scans, query scan history)
  * AI inference proxy (calls internal AI service or Cloudflare AI)

* **Gmail Scanner Worker (separate worker)**

  * Runs scanning flows for users with active Gmail connections
  * Pulls messages via Gmail API, applies pre-filters
  * Emits parsed events into internal pipeline and persists raw email metadata

* **AI Inference Layer (Cloudflare AI / edge model)**

  * Email classification (intent), NER (title, company, location, names), sentiment, follow-up suggestion, summary
  * Fine-tuned models or prompt-engineered LLMs
  * Human-in-loop flags for low-confidence results

* **Database & Storage**

  * Cloudflare D1 (primary relational store) – jobs, users, emails, comments, contacts, OAuth tokens
  * KV/Cache – user preferences, suggestion cache, recently scanned IDs
  * R2 (object storage) – attachments (resumes, job posting PDFs), screenshots
  * Optional Durable Objects / Queues for orchestration & locking

* **Background Jobs & Orchestration**

  * Scan scheduler, re-try queue, follow-up reminder scheduler
  * Worker(s) process email->job matching, enrichment, notification dispatch

* **Notifications & Integrations**

  * Email notifications, in-app Notifications, calendar sync (Google Calendar)
  * Optional webhook/callbacks for third-party integrations (Zapier)

---

# 2) Data model (core tables & example fields)

Below are condensed SQL-like table definitions (use migrations in D1):

**users**

* id TEXT PRIMARY KEY (UUID)
* email TEXT UNIQUE
* name TEXT
* picture TEXT
* timezone TEXT (IANA)
* created_at TIMESTAMP
* settings JSON

**auth_providers**

* id TEXT PK
* user_id TEXT FK → users.id
* provider TEXT (google|oidc|local)
* provider_user_id TEXT
* linked_at TIMESTAMP

**jobs**

* id TEXT PK (UUID)
* user_id TEXT FK
* title TEXT
* company TEXT
* location TEXT
* description TEXT
* status TEXT (enum: open, applied, interviewing, offer, rejected)
* created_at TIMESTAMP
* updated_at TIMESTAMP
* source_email_thread_id TEXT (nullable)
* next_action JSON (due date, type, auto_generated bool)
* metadata JSON (raw extracted fields: salary, posting_url)

**emails**

* id TEXT PK
* user_id TEXT FK
* thread_id TEXT
* gmail_message_id TEXT UNIQUE
* subject TEXT
* from_email TEXT
* to_emails JSON
* cc_emails JSON
* bcc_emails JSON
* snippet TEXT
* body TEXT (optional, indexed as full-text if supported)
* date TIMESTAMP
* is_system BOOLEAN
* parsed_entities JSON (company, title, recruiters[])
* linked_job_id TEXT FK jobs.id (nullable)
* confidence_score FLOAT
* raw_headers JSON
* created_at TIMESTAMP

**contacts**

* id TEXT PK
* user_id TEXT FK
* name TEXT
* emails JSON
* linkedin TEXT
* company TEXT
* last_contacted TIMESTAMP
* metadata JSON

**comments**

* id TEXT PK
* user_id TEXT FK
* job_id TEXT FK
* body TEXT
* created_at TIMESTAMP (timezone-aware)

**oauth_tokens**

* id TEXT PK
* user_id TEXT FK
* provider TEXT
* access_token_enc TEXT (encrypted)
* refresh_token_enc TEXT (encrypted) -- optional: rotate and keep encrypted
* scope TEXT
* expires_at TIMESTAMP
* created_at TIMESTAMP
* last_refresh TIMESTAMP

**attachments (R2 pointers)**

* id TEXT PK
* user_id TEXT FK
* job_id TEXT FK (nullable)
* filename TEXT
* r2_key TEXT
* uploaded_at TIMESTAMP
* version INT

**system_email_domains**

* id TEXT PK
* pattern TEXT (e.g. %.greenhouse.io)
* description TEXT
* created_at TIMESTAMP

---

# 3) AI pipelines — classification & extraction

Design the inference pipeline as discrete stages so you can inspect / re-run / human-correct at each step.

1. **Pre-filter**

   * Reject purely promotional / spam messages by header heuristics
   * Must respect Gmail API quotas and user scopes

2. **Classification**

   * Output: `{ category: one_of([...]), confidence: float }`
   * Categories: `new_posting`, `application_sent`, `interview_invite`, `rejection`, `offer`, `follow_up`, `other`
   * Use conservative thresholds (e.g., require >0.75 to auto-link; 0.5–0.75 show suggestion)

3. **NER / Metadata extraction**

   * Entities: `company`, `title`, `recruiter_name(s)`, `recruiter_email(s)`, `location`, `salary_range`, `posting_url`, `date`(interview)
   * Output: structured JSON + confidence per field

4. **Summarization & Next-Action**

   * Short summary (1–2 lines) + recommended next step (e.g., "follow up in 5 days", "schedule interview")
   * Provide draft follow-up email templates with placeholders

5. **Post-process**

   * Deduplicate entities
   * Normalize company names (case-folding, domain-based heuristics)
   * Domain pattern detection → mark `is_system` true if matches seeded ATS list

6. **Human-in-loop fallback**

   * If overall confidence is low or ambiguous extractions conflict, mark the email for review in UI and provide an “Accept/Reject” overlay to user.

**Inference payload example**

```json
{
  "message_id":"12345",
  "subject":"Interview: Senior Engineer at Acme",
  "body":"Hi John, We'd like to schedule a 30-minute interview..."
}
```

**Inference response**

```json
{
  "category":"interview_invite",
  "confidence":0.92,
  "entities":{
    "company":"Acme, Inc.",
    "title":"Senior Engineer",
    "recruiters":[{"name":"Jane Doe","email":"jane@acme.com"}],
    "date":"2025-11-20T15:00:00-05:00",
    "location":"Zoom",
    "posting_url":null
  },
  "summary":"Acme invited you to a 30-minute interview. Recruiter: Jane Doe.",
  "suggested_next_action":{"type":"confirm_interview","due_in_days":0}
}
```

---

# 4) Email → Job matching heuristics

Build a ranked score combining several signals:

* **Thread match**: existing job has thread_id = email.thread_id → high score.
* **Domain match**: company domain in email.from matches job.company domain → high.
* **Title/Subject fuzzy match**: Levenshtein / token overlap between subject and job.title.
* **Contact match**: recruiter email matches contact linked to job.
* **Time decay**: more recent jobs get slight boost.
* **AI classifier**: if AI predicts the job relates to a specific company/title → add weight.

Use a threshold to auto-link (e.g., score > 0.8), otherwise provide suggestions.

---

# 5) UI/UX: screens and micro-interactions

Prioritize making the most useful flows single-click or automated.

* **Onboarding flow**

  * Connect Gmail, set timezone, choose pre-seeded ATS domains to ignore, upload resume.
  * "First scan" progress UI and sample card creation.

* **Dashboard**

  * Cards with counts, pipeline visualization, quick actions.
  * “Suggested follow-ups” feed at top.

* **Job Details view**

  * Header: title, company, status, next action (with snooze)
  * Timeline: events (email received, comments, status change, interview scheduled)
  * Emails tab: list + AI summary snippets and one-click link to thread / link to job
  * Attachments: resume used, version selector

* **Email Preview**

  * Show AI summary, extracted entities, confidence badges.
  * Suggested job matches with one-click link or create new job pre-filled.
  * “Mark as system email” toggle to correct misclassifications.

* **Contact card**

  * Shows last contact date, emails, LinkedIn link, jobs associated, a quick “email” button (opens compose with template)

* **Next-Action panel**

  * Actions: follow-up templates, schedule interview, mark as rejected, add note
  * “One-click follow up” uses AI draft, editable in a modal

* **Admin / Settings**

  * Manage connected providers, OAuth token refresh logs, data retention settings
  * Kick off re-scan, show token expiry

---

# 6) API contract — example endpoints

Use RESTful endpoints with JWT authorization. Example endpoints:

* `POST /auth/oidc/callback` — link OIDC account
* `POST /auth/local/signup` — register
* `POST /auth/local/login` — login
* `GET /jobs` — list (filters via query params)
* `POST /jobs` — create
* `GET /jobs/:id` — job details (includes timeline)
* `PATCH /jobs/:id` — update status/fields
* `GET /emails` — list scanned emails (filter: linked/unlinked/system)
* `POST /emails/:id/link-job` `{ job_id }` — manual link
* `POST /scanner/sync` — start on-demand scan (worker endpoint)
* `GET /ai/summary?email_id=` — request recompute
* `POST /calendar/sync` — connect calendar

Define event payloads for WebSocket/SSE:

```json
{ "type":"email_scanned", "user_id":"u1", "email_id":"e1", "linked_job_id":"j1" }
```

---

# 7) Security & privacy

* **OAuth + least privilege scopes**

  * Use `https://www.googleapis.com/auth/gmail.readonly` for scanning; for sending follow-ups require `gmail.send` and only request if user enables feature.

* **Token storage**

  * Encrypt access and refresh tokens at rest (AES-256). Use KMS if available.
  * Rotate refresh tokens on use; persist minimal token metadata.

* **Data minimization & user controls**

  * Allow users to opt-out of scanning full email body (only headers/metadata).
  * Allow per-user data retention policy (30/90/365 days or custom delete).
  * Expose a “delete my data” flow that both marks and purges user data.

* **PII handling**

  * Limit logs — redact email bodies in logs.
  * Audit trail for who viewed/changed a job (for shared accounts).

* **Permissions & multi-account linking**

  * Ensure provider linking can map multiple providers into single account; maintain provider metadata.

* **Rate-limit & abuse protection**

  * Backoff for Gmail API quotas; exponential retry and circuit breaker.

---

# 8) Scalability & operational concerns

* **Gmail quotas**: Batch queries carefully; use incremental sync (historyId) and only fetch new messages. For large accounts, allow user to specify date range to scan (default: last 90 days).
* **Workers concurrency**: Use a distributed lock per user (Durable Object or KV lock) to avoid parallel scans for same account.
* **Storage growth**: Move large payloads to R2, keep D1 for metadata only. Index frequently queried fields (user_id, thread_id, linked_job_id).
* **AI inference cost**: Cache AI outputs per email (hash of message) and TTL. Recompute only when user requests or on rescan.
* **Backfills & reprocessing**: Provide admin job to re-run inference across a date range and produce migration-safe outputs.

---

# 9) Testing, QA & monitoring

* **Unit tests**: parsing, fuzzy matching, NER post-processing
* **Integration tests**: Gmail OAuth flow (can use mock servers), API endpoints
* **End-to-end tests**: Full scan → job creation → link → follow-up
* **Observability**:

  * Metrics: emails scanned/day, auto-linked % (accuracy), inference latency, token refresh failures, user error rates
  * Traces for worker flows (scan job lifecycle)
  * Alerts: high failure rate in scanning, sudden drop in inference success, token expiry spike
* **Human feedback loop**: track corrections (user overrides classification) to improve model and heuristics.

---

# 10) Privacy & compliance

* **GDPR/CCPA**:

  * Data export & deletion endpoints
  * Clear consent when connecting Gmail and when giving extra scopes
  * Data processing agreement language for enterprise customers

* **Data retention**:

  * Defaults: keep raw emails 90 days unless user opts into longer retention
  * Option to keep metadata indefinitely while deleting email bodies

---

# 11) Feature roadmap & priorities (deliverables)

I’ll present these as prioritized milestones (no time estimates):

**Milestone A – Core automation & reliability (MVP polish)**

* Implement robust Gmail scanner worker (incremental sync, exponential backoff)
* Basic AI classification + entity extraction pipeline (edge inference)
* Auto-linking by thread/domain + UI suggestions
* Job CRUD + timeline UI for created/linked jobs
* OAuth handling and secure token storage

**Milestone B – User experience & reminders**

* Next-action / reminder system (snooze, scheduled follow-up)
* Calendar integration for interview events
* Attachments to jobs (R2-based resume storage and versioning)
* Improved search/filter over jobs & emails

**Milestone C – Intelligence & analytics**

* AI summarization and draft follow-ups
* Insights dashboard (pipeline metrics, response times)
* Human-in-loop correction and feedback capture for model improvement

**Milestone D – Collaboration & exports**

* Sharing/inviteable job views, comments and attribution
* Data export (CSV/JSON), automatic backups
* Admin re-scan & batch reprocessing

**Milestone E – Advanced**

* Resume-job matching and job recommendation
* LinkedIn / Job board integrations
* Fine-tuning models from collected (consented) labeled data

---

# 12) Example deliverable: follow-up email templates

Provide templates generated by AI with placeholders:

* **Gentle follow-up**

  > Subject: Following up on my application for {{title}} at {{company}}
  > Hi {{recruiter_name}},
  > I hope you’re well — I wanted to check in on the status of my application for {{title}}. I remain very interested in the role and would love to discuss next steps. Please let me know if there is any additional information I can provide.
  > Best,
  > {{user_name}}

* **After interview**

  > Subject: Thank you — {{title}} interview on {{date}}
  > Hi {{interviewer_name}},
  > Thank you for the conversation today — I appreciated learning more about {{company}} and the role. I’m excited about the opportunity to contribute to {{team_or_project}}. Please reach out if you’d like further information.
  > Best,
  > {{user_name}}

Include style variants (concise, formal, casual) and allow user to choose tone.

---

# 13) Metrics to monitor product success (north-star & supporting)

* **North-star**: Active users’ “applications tracked per month” + “follow-ups performed per month”
* **Quality signals**:

  * Auto-link accuracy (user-accepted matches / total suggestions)
  * Time-to-response improvement (did follow-ups improve response rate?)
  * Engagement: jobs with >=1 next-action scheduled
  * Retention: 30-day active user retention
* **Operational**:

  * Scan success rate
  * Inference latency and cost per inference

---

# 14) Migration & DB changes to support new features

Suggested extra migrations:

1. Add `next_action` JSON to `jobs` (if not already present)
2. Add `attachments` table (R2 pointer)
3. Add `is_system` and `confidence_score` to `emails`
4. Add `contacts.last_contacted` and indexing
5. Add `inference_version` to emails (track which model produced results)

---

# 15) Edge cases & tricky scenarios

* **Multiple workers scanning same account** → use per-user lock to prevent duplication.
* **Gmail thread id re-use / archived messages** → keep historyId based sync and fallback to message lists.
* **Recruiter using multiple email addresses** → canonicalize via contact merging and domain heuristics.
* **Emails in foreign languages** → detect language and either use localized NER or fall back to safe heuristics.
* **Conflicting extractions** → present low-confidence results as suggestions; require manual approval to auto-link.

---

# 16) Developer checklist & acceptance criteria for the high-priority features

**Gmail Scanner (acceptance)**

* Successfully completes incremental sync for a Gmail-connected user without fetching unchanged messages.
* Persists email metadata to `emails` table and stores bodies only when permission granted.
* Handles rate limit errors gracefully with exponential backoff.

**AI Extraction (acceptance)**

* Returns classification + entities for sample messages with expected fields.
* Confidence scores stored and surfaced to UI with clear thresholds.
* User can override classification and change linked job; override persisted for model training.

**Auto-linking (acceptance)**

* Auto-links messages to existing jobs when threshold exceeded; user can undo.
* Suggests top 3 candidate jobs for ambiguous matches.

**Next-action Reminders (acceptance)**

* User can set follow-up reminders; they trigger notifications and appear in timeline.
* Reminders can be snoozed and rescheduled.

---

# 17) Small, high-impact UX details

* Show AI confidence inline (low / med / high) with a tooltip “Why we think this is related” highlighting matched tokens/domains.
* “Mark as system” and “Ignore sender for future” actions directly in email UI.
* Keyboard shortcuts for key actions (n = new job, f = follow-up).
* Bulk actions on emails (bulk assign to job, bulk mark as system).

---

# 18) Learning & feedback pipeline

* Store user corrections (classification overrides, entity corrections) in an audit table.
* Periodically export labeled corrections for model fine-tuning (with user consent).
* Track and display model drift metrics (decrease in confidence or increasing number of user corrections).

---

# 19) Final thought: privacy-first defaults = trust

Make privacy the default:

* Default to read-only Gmail scanning (no email sending) and allow opt-in for follow-up email sending.
* Default retention of email body short TTL (e.g., 90 days), expose easy delete/export functionality.
  Users are more likely to adopt a system that clearly explains what is stored and gives them control.


