use dioxus::prelude::*;

use ui::Navbar;
use views::{Blog, Home, Jobs};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[layout(WebNavbar)]
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/jobs")]
    Jobs {},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
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
