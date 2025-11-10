use dioxus::prelude::*;

use ui::{use_auth_provider, Navbar};
use views::{Accounts, Blog, Dashboard, Home, Jobs, Login};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[layout(WebNavbar)]
    #[route("/dashboard")]
    Dashboard {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/jobs")]
    Jobs {},
    #[route("/settings/accounts")]
    Accounts {},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize auth state provider
    let auth = use_auth_provider();

    // Check for OIDC callback on mount
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().expect("no global `window` exists");
            let location = window.location();
            let href = location.href().unwrap_or_default();

            // Check if we're coming back from OIDC callback
            // The callback is handled server-side, so we just need to fetch user
            if !href.contains("error") {
                auth.fetch_user();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = auth;
        }
    });

    rsx! {
        // Global app resources
        document::Stylesheet { href: TAILWIND_CSS }

        // Default metadata
        document::Title { "ApplyMonitor - Track Your Job Applications" }
        document::Meta {
            name: "description",
            content: "Track and manage your job applications effortlessly. Automatically organize applications, interviews, and follow-ups all in one place.",
        }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }
        document::Meta {
            property: "og:title",
            content: "ApplyMonitor - Track Your Job Applications",
        }
        document::Meta {
            property: "og:description",
            content: "Track and manage your job applications effortlessly. Automatically organize applications, interviews, and follow-ups all in one place.",
        }
        document::Meta {
            property: "og:type",
            content: "website",
        }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Dashboard {},
                "Dashboard"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
            Link {
                to: Route::Jobs {},
                "Jobs"
            }
        }

        Outlet::<Route> {}
    }
}
