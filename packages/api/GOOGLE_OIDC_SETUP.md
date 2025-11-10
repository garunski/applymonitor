# Google OIDC Setup Instructions

This guide will walk you through setting up Google OAuth 2.0 (OIDC) for authentication in ApplyMonitor.

## Prerequisites

- A Google account
- Access to [Google Cloud Console](https://console.cloud.google.com/)

## Step 1: Create a Google Cloud Project

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Click the project dropdown at the top
3. Click **"New Project"**
4. Enter a project name (e.g., "ApplyMonitor")
5. Click **"Create"**
6. Wait for the project to be created, then select it

## Step 2: Configure OAuth Consent Screen

**Note**: You no longer need to enable any specific APIs. OAuth/OIDC works directly.

1. Go to **"APIs & Services"** → **"OAuth consent screen"**
2. Select **"External"** (unless you have a Google Workspace account)
3. Click **"Create"**
4. Fill in the required information:
   - **App name**: `ApplyMonitor` (or your app name)
   - **User support email**: Your email address
   - **Developer contact information**: Your email address
5. Click **"Save and Continue"**
6. On the **"Scopes"** page:
   - Click **"Add or Remove Scopes"**
   - In the filter/search box, search for and add:
     - `openid` (OpenID Connect)
     - `email` (See your primary Google Account email address)
     - `profile` (See your personal info, including any personal info you've made publicly available)
   - Click **"Update"**, then **"Save and Continue"**
7. On the **"Test users"** page (if your app is in testing mode):
   - Click **"Add Users"**
   - Add your email address and any other test users
   - Click **"Add"**, then **"Save and Continue"**
8. Review and click **"Back to Dashboard"**

## Step 3: Create OAuth 2.0 Credentials

1. Go to **"APIs & Services"** → **"Credentials"**
2. Click **"Create Credentials"** → **"OAuth client ID"**
3. If prompted, select **"Web application"** as the application type
4. Enter a name (e.g., "ApplyMonitor API")
5. Under **"Authorized redirect URIs"**, click **"Add URI"**
6. Add your callback URL:
   - **For local development**: `http://localhost:8000/auth/callback`
   - **For production**: `https://api.applymonitor.com/auth/callback`
   - (Replace with your actual API URL)
7. Click **"Create"**
8. **IMPORTANT**: Copy the **Client ID** and **Client Secret** immediately
   - You won't be able to see the Client Secret again after closing this dialog
   - If you lose it, you'll need to create new credentials
   - The Client ID looks like: `123456789-abcdefghijklmnop.apps.googleusercontent.com`
   - The Client Secret looks like: `GOCSPX-abcdefghijklmnopqrstuvwxyz`

## Step 4: Configure Local Development

1. Copy the example secrets file:
   ```bash
   cd packages/api
   cp .dev.vars.example .dev.vars
   ```

2. Edit `.dev.vars` and add your credentials:
   ```bash
   OIDC_GOOGLE_CLIENT_ID=your_client_id_here.apps.googleusercontent.com
   OIDC_GOOGLE_CLIENT_SECRET=GOCSPX-your_client_secret_here
   SESSION_SIGNING_KEY=your_secure_random_string_here
   ```

3. Generate a secure signing key:
   ```bash
   openssl rand -hex 32
   ```
   Copy the output and use it as your `SESSION_SIGNING_KEY`

## Step 5: Configure Production Secrets

For production deployment, use Wrangler to set secrets:

```bash
cd packages/api

# Set Google OIDC credentials
wrangler secret put OIDC_GOOGLE_CLIENT_ID
# Paste your Client ID when prompted (e.g., 123456789-abc.apps.googleusercontent.com)

wrangler secret put OIDC_GOOGLE_CLIENT_SECRET
# Paste your Client Secret when prompted (e.g., GOCSPX-abc123...)

# Set session signing key
wrangler secret put SESSION_SIGNING_KEY
# Paste your secure random string when prompted
```

## Step 6: Update Authorized Redirect URIs

Make sure your redirect URIs match your actual deployment:

- **Local**: `http://localhost:8000/auth/callback`
- **Production**: `https://api.applymonitor.com/auth/callback`

To update:
1. Go to **"APIs & Services"** → **"Credentials"**
2. Click on your OAuth 2.0 Client ID
3. Under **"Authorized redirect URIs"**, add/update your URIs
4. Click **"Save"**

## Testing

1. Start your local development server:
   ```bash
   cd packages/api
   wrangler dev
   ```

2. Visit `http://localhost:8000/auth/login?provider=google`
3. You should be redirected to Google's login page
4. After logging in, you'll be redirected back to your callback URL

## Troubleshooting

### "redirect_uri_mismatch" Error
- Make sure your redirect URI in Google Cloud Console exactly matches the one in your code
- Check for trailing slashes, http vs https, and port numbers
- The URI must match character-for-character

### "invalid_client" Error
- Verify your Client ID and Client Secret are correct
- Make sure you're using the right credentials for the right environment
- Check that you copied the entire Client ID and Client Secret

### "OIDC_GOOGLE_CLIENT_ID secret not found"
- Make sure `.dev.vars` exists in `packages/api/` directory
- Check that the file contains the correct variable names (no typos)
- For production, make sure you've set the secrets with `wrangler secret put`

### OAuth Consent Screen Issues
- If you see "This app isn't verified", you may need to:
  - Add test users in the OAuth consent screen (for testing mode)
  - Complete the OAuth verification process (for production apps with external users)
- For testing, make sure you've added yourself as a test user

### "access_denied" Error
- Make sure you've added yourself as a test user in the OAuth consent screen
- Check that your app is in "Testing" mode (not "In production")
- Verify the scopes are correctly configured

## Security Best Practices

1. **Never commit `.dev.vars` to version control** (already in `.gitignore`)
2. **Use different OAuth credentials for development and production**
3. **Rotate secrets regularly** (especially if exposed)
4. **Use strong, random signing keys** (generate with `openssl rand -hex 32`)
5. **Limit OAuth scopes** to only what you need (`openid`, `email`, `profile`)
6. **Keep your Client Secret secure** - treat it like a password
7. **Use HTTPS in production** - never use HTTP for OAuth callbacks in production

## Additional Resources

- [Google OAuth 2.0 Documentation](https://developers.google.com/identity/protocols/oauth2)
- [OpenID Connect Documentation](https://openid.net/connect/)
- [Google Cloud Console](https://console.cloud.google.com/)
- [Cloudflare Workers Secrets](https://developers.cloudflare.com/workers/configuration/secrets/)
