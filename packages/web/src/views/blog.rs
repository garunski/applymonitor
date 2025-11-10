use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    let title = format!("Blog #{id} - ApplyMonitor");
    
    rsx! {
        document::Title { "{title}" }
        document::Meta {
            name: "description",
            content: "Blog post about job application tracking and career development.",
        }
        document::Meta {
            property: "og:title",
            content: "{title}",
        }
        document::Meta {
            property: "og:description",
            content: "Blog post about job application tracking and career development.",
        }

        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}
