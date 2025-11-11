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
            class: "px-4 sm:px-6 lg:px-8 py-6",
            div {
                class: "max-w-4xl mx-auto",
                h1 {
                    class: "text-3xl font-bold text-gray-900 dark:text-white mb-6",
                    "This is blog #{id}!"
                }
                p {
                    class: "text-gray-700 dark:text-gray-300 mb-6",
                    "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
                }
                div {
                    class: "flex gap-4",
                    Link {
                        to: Route::Blog { id: id - 1 },
                        class: "text-brand-600 hover:text-brand-500 dark:text-brand-400 dark:hover:text-brand-300",
                        "Previous"
                    }
                    span {
                        class: "text-gray-500 dark:text-gray-400",
                        " <---> "
                    }
                    Link {
                        to: Route::Blog { id: id + 1 },
                        class: "text-brand-600 hover:text-brand-500 dark:text-brand-400 dark:hover:text-brand-300",
                        "Next"
                    }
                }
            }
        }
    }
}
