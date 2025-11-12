//! HTTP client with credentials support for cross-origin requests

use crate::services::error::ServiceError;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Request, RequestCredentials, RequestInit, RequestMode, Response};

/// Make a GET request with credentials
pub async fn get(url: &str) -> Result<Response, ServiceError> {
    request(url, "GET", None).await
}

/// Make a POST request with credentials
pub async fn post(url: &str, body: Option<&str>) -> Result<Response, ServiceError> {
    request(url, "POST", body).await
}

/// Make a PUT request with credentials
pub async fn put(url: &str, body: Option<&str>) -> Result<Response, ServiceError> {
    request(url, "PUT", body).await
}

/// Make a DELETE request with credentials
pub async fn delete(url: &str) -> Result<Response, ServiceError> {
    request(url, "DELETE", None).await
}

/// Make a PATCH request with credentials
pub async fn patch(url: &str, body: Option<&str>) -> Result<Response, ServiceError> {
    request(url, "PATCH", body).await
}

/// Internal function to make HTTP requests with credentials
async fn request(url: &str, method: &str, body: Option<&str>) -> Result<Response, ServiceError> {
    let opts = RequestInit::new();
    opts.set_method(method);
    opts.set_mode(RequestMode::Cors);
    opts.set_credentials(RequestCredentials::Include); // Include cookies in cross-origin requests

    match body {
        Some(body_str) => {
            let body_value = JsValue::from_str(body_str);
            opts.set_body(&body_value);
        }
        None => {
            opts.set_body(&JsValue::NULL);
        }
    }

    let request = Request::new_with_str_and_init(url, &opts)
        .map_err(|e| ServiceError::Network(format!("Failed to create request: {:?}", e)))?;

    // Only set Content-Type if there's a body
    if body.is_some() {
        request
            .headers()
            .set("Content-Type", "application/json")
            .map_err(|e| ServiceError::Network(format!("Failed to set Content-Type: {:?}", e)))?;
    }

    let window =
        web_sys::window().ok_or_else(|| ServiceError::Network("No window object".to_string()))?;

    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| ServiceError::Network(format!("Request failed: {:?}", e)))?;

    let resp: Response = resp_value
        .dyn_into::<Response>()
        .map_err(|e| ServiceError::Network(format!("Response is not a Response: {:?}", e)))?;

    Ok(resp)
}

/// Helper to parse JSON response
pub async fn json<T>(response: Response) -> Result<T, ServiceError>
where
    T: serde::de::DeserializeOwned,
{
    let text = response
        .text()
        .map_err(|e| ServiceError::Parse(format!("Failed to get response text: {:?}", e)))?;

    let text_value = wasm_bindgen_futures::JsFuture::from(text)
        .await
        .map_err(|e| ServiceError::Parse(format!("Failed to read response text: {:?}", e)))?;

    let text_str = text_value
        .as_string()
        .ok_or_else(|| ServiceError::Parse("Response text is not a string".to_string()))?;

    serde_json::from_str(&text_str)
        .map_err(|e| ServiceError::Parse(format!("Failed to parse JSON: {}", e)))
}

/// Helper to get response text
pub async fn text(response: Response) -> Result<String, ServiceError> {
    let text = response
        .text()
        .map_err(|e| ServiceError::Parse(format!("Failed to get response text: {:?}", e)))?;

    let text_value = wasm_bindgen_futures::JsFuture::from(text)
        .await
        .map_err(|e| ServiceError::Parse(format!("Failed to read response text: {:?}", e)))?;

    text_value
        .as_string()
        .ok_or_else(|| ServiceError::Parse("Response text is not a string".to_string()))
}
