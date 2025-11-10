# ApplyMonitor API

Cloudflare Workers API package using worker-rs with D1 database support.

## Local Development Setup

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Wrangler CLI (v4+)
- Node.js 18+

### Initial Setup

1. **Create the D1 database** (if not already created):
   ```bash
   cd packages/api
   wrangler d1 create applymonitor-db
   ```
   
   This will output a database ID. Update `wrangler.toml` with the actual database ID.

2. **Apply migrations locally**:
   ```bash
   wrangler d1 migrations apply applymonitor-db --local
   ```

3. **Start local development server**:
   ```bash
   wrangler dev
   ```
   
   The API will be available at `http://localhost:8787`

### Testing

- Root endpoint: `GET http://localhost:8787/`
- Test endpoint: `GET http://localhost:8787/test`

The test endpoint creates a table, inserts a test item, and returns all items.

## Production Deployment

1. **Apply migrations to production**:
   ```bash
   wrangler d1 migrations apply applymonitor-db --remote
   ```

2. **Deploy the worker**:
   ```bash
   wrangler deploy
   ```

## OIDC Authentication Setup

For detailed step-by-step instructions, see [GOOGLE_OIDC_SETUP.md](./GOOGLE_OIDC_SETUP.md).

### Quick Setup

1. **Create Google OAuth Application**:
   - Go to [Google Cloud Console](https://console.cloud.google.com/)
   - Create a new project
   - Configure OAuth consent screen (no API enabling needed)
   - Create OAuth 2.0 Client ID credentials
   - Set authorized redirect URI to: `https://api.applymonitor.com/auth/callback` (or your API URL)
   - Save the Client ID and Client Secret

### Setting Up Secrets

Set the following secrets using Wrangler:

```bash
# Google OIDC credentials
wrangler secret put OIDC_GOOGLE_CLIENT_ID
wrangler secret put OIDC_GOOGLE_CLIENT_SECRET

# Session signing key (generate a secure random string)
wrangler secret put SESSION_SIGNING_KEY
```

For local development, create a `.dev.vars` file in the `packages/api` directory:

1. **Copy the example file:**
   ```bash
   cd packages/api
   cp .dev.vars.example .dev.vars
   ```

2. **Fill in your actual secrets:**
   ```bash
   # Edit .dev.vars with your actual values
   OIDC_GOOGLE_CLIENT_ID=your_actual_client_id
   OIDC_GOOGLE_CLIENT_SECRET=your_actual_client_secret
   SESSION_SIGNING_KEY=your_actual_signing_key
   ```

3. **Generate a secure signing key:**
   ```bash
   # Generate a secure random 32-byte hex string
   openssl rand -hex 32
   ```

**Security Notes:**
- `.dev.vars` is automatically ignored by git (in `.gitignore`)
- Never commit `.dev.vars` to version control
- Use `.dev.vars.example` as a template for team members
- For production, use `wrangler secret put` instead

### Environment Variables

The following variables are configured in `wrangler.toml`:

- `JWT_ISSUER`: JWT issuer URL (default: `https://api.applymonitor.com`)
- `SESSION_COOKIE_NAME`: Name of the session cookie (default: `session`)
- `FRONTEND_URL`: Frontend URL for redirects after authentication (default: `https://applymonitor.com`)

### Authentication Flow

1. User clicks "Sign in with Google" button
2. Frontend redirects to `/auth/login?provider=google`
3. Worker redirects to Google OAuth with state/nonce for CSRF protection
4. Google redirects to `/auth/callback?code=...&state=...`
5. Worker validates state, exchanges code for ID token
6. Worker validates ID token signature and claims
7. Worker upserts user in D1 database
8. Worker creates 24-hour session JWT, sets HttpOnly cookie
9. Worker redirects to frontend
10. Frontend fetches user via `/api/me` endpoint

### Protected Routes

All routes except the following are protected and require authentication:

- `GET /` (root/landing page)
- `GET /health`
- `GET /auth/login`
- `GET /auth/callback`
- `GET /auth/logout`

Protected routes return `401 Unauthorized` if no valid session cookie is present.

### API Endpoints

- `GET /auth/login?provider=google` - Initiates OIDC login flow
- `GET /auth/callback?code=...&state=...` - OIDC callback handler
- `GET /auth/logout` - Logs out user and clears session cookie
- `GET /api/me` - Returns current authenticated user (protected)

## Important Notes

- Wrangler v4 defaults to `--local` mode. Use `--remote` flag for production operations.
- Local D1 database data is persisted in `.wrangler/state/d1/` directory.
- Make sure to update `wrangler.toml` with your actual database IDs before deploying.
- Session cookies are HttpOnly, Secure, and SameSite=Strict for security.
- Session tokens expire after 24 hours.

