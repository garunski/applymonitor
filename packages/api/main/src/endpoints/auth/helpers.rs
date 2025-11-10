use worker::*;

pub fn extract_cookie_value(cookie_header: &str, name: &str) -> Option<String> {
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if cookie.starts_with(&format!("{}=", name)) {
            return Some(cookie[name.len() + 1..].to_string());
        }
    }
    None
}

pub fn get_api_base_url(env: &Env) -> worker::Result<String> {
    // Try to get from environment variable or use default
    if let Ok(url) = env.var("API_BASE_URL") {
        return Ok(url.to_string());
    }

    // Default to production URL
    Ok("https://api.applymonitor.com".to_string())
}

pub fn is_secure_cookie(env: &Env) -> bool {
    // Check if API_BASE_URL starts with https://
    if let Ok(url) = env.var("API_BASE_URL") {
        return url.to_string().starts_with("https://");
    }
    // Default to secure for production
    true
}
