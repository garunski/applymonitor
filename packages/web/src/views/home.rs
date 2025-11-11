use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsBarChart, BsCalendar, BsEnvelope, BsFileText, BsRobot, BsSearch, BsStars},
    Icon,
};
use ui::components::button::{Button, ButtonVariant};

const LOGO_SVG: Asset = asset!("/assets/logo.svg");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "ApplyMonitor - Track Your Job Applications" }
        document::Meta {
            name: "description",
            content: "Stop juggling spreadsheets and emails. ApplyMonitor automatically organizes your job search. Connect Gmail, track applications, schedule follow-ups, and land your dream job—all in one place.",
        }
        document::Meta {
            property: "og:title",
            content: "ApplyMonitor - Track Your Job Applications",
        }
        document::Meta {
            property: "og:description",
            content: "Stop juggling spreadsheets and emails. ApplyMonitor automatically organizes your job search. Connect Gmail, track applications, schedule follow-ups, and land your dream job—all in one place.",
        }
        document::Meta {
            property: "og:type",
            content: "website",
        }

        div {
            class: "min-h-screen bg-gradient-to-br from-slate-50 via-brand-50 to-brand-100 dark:from-gray-950 dark:via-gray-900 dark:to-brand-900",
            // Hero Section with Asymmetric Layout
            div {
                class: "relative overflow-hidden",
                div {
                    class: "absolute inset-0 opacity-5",
                    style: "background-image: radial-gradient(circle, #64748b 1px, transparent 1px); background-size: 20px 20px;",
                }
                div {
                    class: "relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-20 pb-16 sm:pt-32 sm:pb-24",
                    div {
                        class: "lg:grid lg:grid-cols-12 lg:gap-8 items-center",
                        // Left Column - Text Content
                        div {
                            class: "lg:col-span-7",
                            div {
                                class: "flex items-center gap-4 mb-8",
                                img {
                                    src: LOGO_SVG,
                                    alt: "ApplyMonitor",
                                    class: "h-12 w-12",
                                }
                                h1 {
                                    class: "text-4xl font-bold",
                                    span {
                                        class: "text-brand-500",
                                        "Apply"
                                    }
                                    " "
                                    span {
                                        class: "text-brand-900 dark:text-white",
                                        "Monitor"
                                    }
                                }
                            }
                            div {
                                class: "inline-flex items-center rounded-full px-4 py-1.5 text-sm font-medium bg-brand-100 dark:bg-brand-900/30 text-brand-800 dark:text-brand-200 mb-6",
                                Icon {
                                    class: "mr-2",
                                    width: 16,
                                    height: 16,
                                    fill: "currentColor",
                                    icon: BsStars,
                                }
                                "Never lose track of an application again"
                            }
                            h1 {
                                class: "text-5xl sm:text-6xl lg:text-7xl font-extrabold tracking-tight text-gray-900 dark:text-white mb-6",
                                span {
                                    class: "block",
                                    "Stop juggling"
                                }
                                span {
                                    class: "block bg-gradient-to-r from-brand-500 to-brand-600 dark:from-brand-400 dark:to-brand-500 bg-clip-text text-transparent",
                                    "spreadsheets"
                                }
                                span {
                                    class: "block",
                                    "and emails."
                                }
                            }
                            p {
                                class: "text-xl text-gray-600 dark:text-gray-300 mb-8 leading-relaxed max-w-2xl",
                                "ApplyMonitor automatically organizes your job search. Connect Gmail, track applications, schedule follow-ups, and land your dream job—all in one place."
                            }
                            div {
                                class: "flex flex-col sm:flex-row gap-4",
                                Link {
                                    to: Route::Login {},
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        class: "text-lg px-8 py-3",
                                        "Get Started →"
                                    }
                                }
                            }
                        }
                        // Right Column - Visual Element
                        div {
                            class: "mt-12 lg:mt-0 lg:col-span-5",
                            div {
                                class: "relative",
                                // Stats Cards
                                div {
                                    class: "grid grid-cols-2 gap-4",
                                    div {
                                        class: "bg-white/80 dark:bg-gray-800/80 backdrop-blur-sm rounded-2xl p-6 shadow-lg border border-gray-200/50 dark:border-gray-700/50",
                                        div {
                                            class: "text-3xl font-bold text-brand-500 dark:text-brand-400 mb-1",
                                            "100+"
                                        }
                                        div {
                                            class: "text-sm text-gray-600 dark:text-gray-400",
                                            "Applications Tracked"
                                        }
                                    }
                                    div {
                                        class: "bg-white/80 dark:bg-gray-800/80 backdrop-blur-sm rounded-2xl p-6 shadow-lg border border-gray-200/50 dark:border-gray-700/50",
                                        div {
                                            class: "text-3xl font-bold text-brand-600 dark:text-brand-500 mb-1",
                                            "24/7"
                                        }
                                        div {
                                            class: "text-sm text-gray-600 dark:text-gray-400",
                                            "Auto Monitoring"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Features Section - Two Column Layout
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24",
                div {
                    class: "text-center mb-16",
                    h2 {
                        class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                        "Everything you need"
                    }
                    p {
                        class: "text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto",
                        "Powerful features that work together to streamline your job search"
                    }
                }
                div {
                    class: "grid md:grid-cols-2 gap-8",
                    // Feature 1 - Large Card
                    div {
                        class: "md:col-span-2 bg-gradient-to-br from-brand-500 to-brand-600 rounded-3xl p-8 md:p-12 text-white shadow-2xl",
                        div {
                            class: "flex items-start gap-6",
                            div {
                                class: "flex-shrink-0 w-16 h-16 bg-white/20 rounded-2xl flex items-center justify-center backdrop-blur-sm",
                                Icon {
                                    width: 32,
                                    height: 32,
                                    fill: "currentColor",
                                    icon: BsEnvelope,
                                }
                            }
                            div {
                                class: "flex-1",
                                h3 {
                                    class: "text-2xl font-bold mb-3",
                                    "Smart Email Integration"
                                }
                                p {
                                    class: "text-brand-100 text-lg leading-relaxed",
                                    "Connect your Gmail and watch ApplyMonitor automatically detect, classify, and organize job-related emails. Application confirmations, interview requests, rejections—we've got it covered. No manual entry required."
                                }
                            }
                        }
                    }
                    // Feature 2
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-2xl p-8 shadow-lg border border-gray-200 dark:border-gray-700 hover:shadow-xl transition-shadow",
                        div {
                            class: "w-12 h-12 bg-brand-100 dark:bg-brand-900/30 rounded-xl flex items-center justify-center mb-4",
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsBarChart,
                            }
                        }
                        h3 {
                            class: "text-xl font-bold text-gray-900 dark:text-white mb-3",
                            "Application Pipeline"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300 leading-relaxed",
                            "Visualize your entire job search pipeline. Track applications from initial submission through interviews, offers, and decisions. Never wonder \"where did I apply?\" again."
                        }
                    }
                    // Feature 3
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-2xl p-8 shadow-lg border border-gray-200 dark:border-gray-700 hover:shadow-xl transition-shadow",
                        div {
                            class: "w-12 h-12 bg-brand-200 dark:bg-brand-800/30 rounded-xl flex items-center justify-center mb-4",
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsRobot,
                            }
                        }
                        h3 {
                            class: "text-xl font-bold text-gray-900 dark:text-white mb-3",
                            "AI Follow-up Assistant"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300 leading-relaxed",
                            "Get personalized follow-up email suggestions powered by AI. Multiple tone options—professional, friendly, or persistent. One click to send and track."
                        }
                    }
                    // Feature 4
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-2xl p-8 shadow-lg border border-gray-200 dark:border-gray-700 hover:shadow-xl transition-shadow",
                        div {
                            class: "w-12 h-12 bg-green-100 dark:bg-green-900/30 rounded-xl flex items-center justify-center mb-4",
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsCalendar,
                            }
                        }
                        h3 {
                            class: "text-xl font-bold text-gray-900 dark:text-white mb-3",
                            "Calendar Sync"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300 leading-relaxed",
                            "Automatically sync interview dates with Google Calendar. Get reminders and keep your schedule organized without switching apps."
                        }
                    }
                    // Feature 5
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-2xl p-8 shadow-lg border border-gray-200 dark:border-gray-700 hover:shadow-xl transition-shadow",
                        div {
                            class: "w-12 h-12 bg-purple-100 dark:bg-purple-900/30 rounded-xl flex items-center justify-center mb-4",
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsFileText,
                            }
                        }
                        h3 {
                            class: "text-xl font-bold text-gray-900 dark:text-white mb-3",
                            "Interview Journal"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300 leading-relaxed",
                            "Keep detailed notes on every interview. What went well, what didn't, questions asked. Build your knowledge base and improve with each opportunity."
                        }
                    }
                    // Feature 6
                    div {
                        class: "bg-white dark:bg-gray-800 rounded-2xl p-8 shadow-lg border border-gray-200 dark:border-gray-700 hover:shadow-xl transition-shadow",
                        div {
                            class: "w-12 h-12 bg-orange-100 dark:bg-orange-900/30 rounded-xl flex items-center justify-center mb-4",
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: BsSearch,
                            }
                        }
                        h3 {
                            class: "text-xl font-bold text-gray-900 dark:text-white mb-3",
                            "Semantic Search"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300 leading-relaxed",
                            "Find anything instantly. \"Show me Rust jobs\" or \"Where did I mention my internship?\" Our AI understands context, not just keywords."
                        }
                    }
                }
            }

            // CTA Section - Modern Design
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24",
                div {
                    class: "relative overflow-hidden rounded-3xl bg-gradient-to-r from-brand-500 via-brand-600 to-brand-700 p-12 md:p-16 shadow-2xl",
                    div {
                        class: "absolute inset-0 opacity-10",
                        style: "background-image: radial-gradient(circle, white 1px, transparent 1px); background-size: 30px 30px;",
                    }
                    div {
                        class: "relative text-center",
                        h2 {
                            class: "text-4xl md:text-5xl font-bold text-white mb-4",
                            "Ready to transform your job search?"
                        }
                        p {
                            class: "text-xl text-brand-100 mb-8 max-w-2xl mx-auto",
                            "Join thousands of job seekers who've streamlined their application process. Get started in seconds."
                        }
                        div {
                            class: "flex flex-col sm:flex-row gap-4 justify-center",
                            Link {
                                to: Route::Login {},
                                Button {
                                    variant: ButtonVariant::Primary,
                                    class: "bg-white text-brand-600 hover:bg-brand-50 text-lg px-8 py-3 shadow-lg",
                                    "Get Started →"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
