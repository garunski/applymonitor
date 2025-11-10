//! API configuration

/// Get the base URL for the API
/// Returns `http://localhost:8000` for development or `https://api.applymonitor.com` for production
pub fn get_api_base_url() -> String {
    // For now, use localhost for development
    // In the future, this could check environment variables or build-time configuration
    "http://localhost:8000".to_string()
}
