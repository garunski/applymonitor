use dioxus::prelude::*;

#[component]
pub fn ApiTest() -> Element {
    let mut api_url = use_signal(|| "http://localhost:8000".to_string());
    let mut status = use_signal(|| "Ready".to_string());
    let mut root_response = use_signal(|| String::new());
    let mut test_response = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    rsx! {
        div {
            style: "padding: 2rem; max-width: 800px; margin: 0 auto;",
            h2 { "API Test" }
            
            div {
                style: "margin-bottom: 1rem;",
                label {
                    style: "display: block; margin-bottom: 0.5rem; font-weight: bold;",
                    "API URL:"
                }
                input {
                    r#type: "text",
                    value: "{api_url}",
                    oninput: move |e| *api_url.write() = e.value(),
                    style: "width: 100%; padding: 0.5rem; border: 1px solid #ccc; border-radius: 4px;",
                    placeholder: "http://localhost:8000"
                }
            }

            div {
                style: "margin-bottom: 1rem;",
                p {
                    style: "padding: 0.5rem; background: #f0f0f0; border-radius: 4px;",
                    "Status: {status}"
                }
            }

            div {
                style: "display: flex; gap: 1rem; margin-bottom: 2rem;",
                button {
                    disabled: loading(),
                    onclick: move |_| {
                        spawn(async move {
                            *loading.write() = true;
                            *status.write() = "Testing root endpoint...".to_string();
                            
                            let url = format!("{}/", api_url());
                            match gloo_net::http::Request::get(&url).send().await {
                                Ok(response) => {
                                    match response.text().await {
                                        Ok(text) => {
                                            *root_response.write() = text;
                                            *status.write() = "Root endpoint: Success".to_string();
                                        }
                                        Err(e) => {
                                            *root_response.write() = format!("Error reading response: {}", e);
                                            *status.write() = "Root endpoint: Error".to_string();
                                        }
                                    }
                                }
                                Err(e) => {
                                    *root_response.write() = format!("Error: {}", e);
                                    *status.write() = "Root endpoint: Error".to_string();
                                }
                            }
                            *loading.write() = false;
                        });
                    },
                    style: "padding: 0.75rem 1.5rem; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    "Test Root (/)"
                }
                
                button {
                    disabled: loading(),
                    onclick: move |_| {
                        spawn(async move {
                            *loading.write() = true;
                            *status.write() = "Testing /test endpoint...".to_string();
                            
                            let url = format!("{}/test", api_url());
                            match gloo_net::http::Request::get(&url).send().await {
                                Ok(response) => {
                                    match response.text().await {
                                        Ok(text) => {
                                            *test_response.write() = text;
                                            *status.write() = "Test endpoint: Success".to_string();
                                        }
                                        Err(e) => {
                                            *test_response.write() = format!("Error reading response: {}", e);
                                            *status.write() = "Test endpoint: Error".to_string();
                                        }
                                    }
                                }
                                Err(e) => {
                                    *test_response.write() = format!("Error: {}", e);
                                    *status.write() = "Test endpoint: Error".to_string();
                                }
                            }
                            *loading.write() = false;
                        });
                    },
                    style: "padding: 0.75rem 1.5rem; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    "Test /test"
                }
            }

            if !root_response().is_empty() {
                div {
                    style: "margin-bottom: 1.5rem;",
                    h3 { "Root Endpoint Response:" }
                    pre {
                        style: "background: #ffffff; color: #000000; padding: 1rem; border: 1px solid #dee2e6; border-radius: 4px; overflow-x: auto; white-space: pre-wrap; font-family: monospace;",
                        "{root_response}"
                    }
                }
            }

            if !test_response().is_empty() {
                div {
                    h3 { "Test Endpoint Response:" }
                    pre {
                        style: "background: #ffffff; color: #000000; padding: 1rem; border: 1px solid #dee2e6; border-radius: 4px; overflow-x: auto; white-space: pre-wrap; font-family: monospace;",
                        "{test_response}"
                    }
                }
            }
        }
    }
}

