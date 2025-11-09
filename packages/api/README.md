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

## Important Notes

- Wrangler v4 defaults to `--local` mode. Use `--remote` flag for production operations.
- Local D1 database data is persisted in `.wrangler/state/d1/` directory.
- Make sure to update `wrangler.toml` with your actual database IDs before deploying.

