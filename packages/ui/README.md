# UI

This crate contains all shared components for the workspace. This is a great place to place any UI you would like to use in multiple platforms like a common `Button` or `Navbar` component.

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform
│  ├─ echo.rs # The shared echo component that communicates with the server
│  ├─ navbar.rs # The Navbar component that will be used in the layout of every platform's router
```

## Dependencies

Since this crate is shared between multiple platforms, it should not pull in any platform specific dependencies. For example, if you want to use the `web_sys` crate in the web build of your app, you should not add it to this crate. Instead, you should add platform specific dependencies to the [web](../web/Cargo.toml), [desktop](../desktop/Cargo.toml), or [mobile](../mobile/Cargo.toml) crates.

## API Configuration

The API endpoint is configurable via environment variables or compile-time features.

### Runtime Configuration (Environment Variables)

Set the `API_BASE_URL` environment variable to override the default API endpoint:

```bash
export API_BASE_URL=https://api.example.com
dx serve  # or your build command
```

### Compile-time Configuration (Features)

You can also configure the API URL at compile time using Cargo features:

```bash
# Development (default)
cargo build --features api-dev

# Staging
cargo build --features api-staging

# Production
cargo build --features api-prod
```

Or in your `Cargo.toml`:

```toml
[dependencies]
ui = { workspace = true, features = ["api-prod"] }
```

**Priority order:**
1. `API_BASE_URL` environment variable (highest priority)
2. Compile-time features (`api-prod`, `api-staging`, `api-dev`)
3. Default: `http://localhost:8000` (development)
