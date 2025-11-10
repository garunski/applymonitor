pub fn get_api_base_url() -> String {
    if let Ok(url) = std::env::var("API_BASE_URL") {
        if !url.is_empty() {
            return url;
        }
    }

    #[cfg(feature = "api-prod")]
    {
        return "https://api.applymonitor.com".to_string();
    }

    #[cfg(feature = "api-staging")]
    {
        return "https://api-staging.applymonitor.com".to_string();
    }

    "http://localhost:8000".to_string()
}
