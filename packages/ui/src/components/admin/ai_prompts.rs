//! AI Prompts editor component

use crate::services::ai_service::{AiPrompt, AiService};
use dioxus::prelude::*;

#[component]
pub fn AiPromptsEditor() -> Element {
    let mut active_tab = use_signal(|| "classify".to_string());
    let prompts = use_signal(Vec::<AiPrompt>::new);
    let loading = use_signal(|| true);
    let error = use_signal(|| None::<String>);
    let mut prompt_text = use_signal(String::new);
    let mut prompt_name = use_signal(String::new);

    let load_prompts = {
        let mut loading = loading;
        let mut error = error;
        let mut prompts = prompts;
        move |stage: String| {
            spawn(async move {
                *loading.write() = true;
                *error.write() = None;

                match AiService::list_prompts(Some(&stage)).await {
                    Ok(loaded_prompts) => {
                        *prompts.write() = loaded_prompts;
                        *error.write() = None;
                    }
                    Err(e) => {
                        *error.write() = Some(format!("Failed to load prompts: {:?}", e));
                    }
                }

                *loading.write() = false;
            });
        }
    };

    use_effect(move || {
        load_prompts(active_tab());
    });

    let active_prompt = prompts().iter().find(|p| p.is_active).cloned();
    let prompts_list = prompts().clone();

    rsx! {
        div {
            class: "space-y-6",
            // Tabs
            div {
                class: "border-b border-gray-200 dark:border-gray-700",
                nav {
                    class: "-mb-px flex space-x-8",
                    TabButton {
                        label: "Classify",
                        stage: "classify",
                        active: active_tab() == "classify",
                        onclick: move |_| {
                            *active_tab.write() = "classify".to_string();
                            load_prompts("classify".to_string());
                        }
                    }
                    TabButton {
                        label: "Extract",
                        stage: "extract",
                        active: active_tab() == "extract",
                        onclick: move |_| {
                            *active_tab.write() = "extract".to_string();
                            load_prompts("extract".to_string());
                        }
                    }
                    TabButton {
                        label: "Summarize",
                        stage: "summarize",
                        active: active_tab() == "summarize",
                        onclick: move |_| {
                            *active_tab.write() = "summarize".to_string();
                            load_prompts("summarize".to_string());
                        }
                    }
                }
            }

            if let Some(err) = error() {
                div {
                    class: "p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md",
                    p {
                        class: "text-sm text-red-800 dark:text-red-200",
                        {err}
                    }
                }
            }

            if loading() {
                div {
                    class: "text-center py-8",
                    "Loading prompts..."
                }
            } else {
                div {
                    class: "space-y-6",
                    // Active prompt display
                    if let Some(ref active) = active_prompt {
                        div {
                            class: "p-5 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700",
                            div {
                                class: "flex items-center justify-between mb-4",
                                div {
                                    h3 {
                                        class: "text-lg font-medium text-gray-900 dark:text-gray-100",
                                        {active.name.clone()}
                                    }
                                    p {
                                        class: "text-sm text-gray-500 dark:text-gray-400",
                                        "Active version"
                                    }
                                }
                                span {
                                    class: "px-3 py-1 text-xs font-medium bg-green-100 dark:bg-green-900/20 text-green-800 dark:text-green-200 rounded-full",
                                    "Active"
                                }
                            }
                            pre {
                                class: "p-4 bg-gray-50 dark:bg-gray-900 rounded-md text-sm text-gray-800 dark:text-gray-200 whitespace-pre-wrap font-mono",
                                {active.prompt.clone()}
                            }
                        }
                    }

                    // Editor
                    div {
                        class: "p-5 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700",
                        h3 {
                            class: "text-lg font-medium text-gray-900 dark:text-gray-100 mb-4",
                            "Create New Version"
                        }
                        div {
                            class: "space-y-4",
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                    "Prompt Name"
                                }
                                input {
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                    r#type: "text",
                                    placeholder: "e.g., Classification v2",
                                    value: prompt_name(),
                                    oninput: move |e| *prompt_name.write() = e.value(),
                                }
                            }
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                    "Prompt Text"
                                }
                                textarea {
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 font-mono text-sm",
                                    rows: 15,
                                    placeholder: "Enter prompt template with {{variables}}...",
                                    value: prompt_text(),
                                    oninput: move |e| *prompt_text.write() = e.value(),
                                }
                            }
                            div {
                                class: "flex gap-3",
                                button {
                                    class: "px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50",
                                    disabled: prompt_name().is_empty() || prompt_text().is_empty(),
                                    onclick: {
                                        let mut prompt_name = prompt_name;
                                        let mut prompt_text = prompt_text;
                                        let mut error = error;
                                        let mut loading = loading;
                                        let mut prompts = prompts;
                                        move |_| {
                                            let name = prompt_name();
                                            let text = prompt_text();
                                            let stage = active_tab();
                                            spawn(async move {
                                                *error.write() = None;
                                                match AiService::create_prompt(
                                                    &name,
                                                    &stage,
                                                    &text,
                                                ).await {
                                                    Ok(_) => {
                                                        *prompt_name.write() = String::new();
                                                        *prompt_text.write() = String::new();
                                                        *loading.write() = true;
                                                        *error.write() = None;
                                                        match AiService::list_prompts(Some(&stage)).await {
                                                            Ok(loaded_prompts) => {
                                                                *prompts.write() = loaded_prompts;
                                                            }
                                                            Err(e) => {
                                                                *error.write() = Some(format!("Failed to reload prompts: {:?}", e));
                                                            }
                                                        }
                                                        *loading.write() = false;
                                                    }
                                                    Err(e) => {
                                                        *error.write() = Some(format!("Failed to create prompt: {:?}", e));
                                                    }
                                                }
                                            });
                                        }
                                    },
                                    "Save as New Version"
                                }
                                if let Some(ref active) = active_prompt {
                                    button {
                                        class: "px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700",
                                        onclick: {
                                            let active_id = active.id.clone();
                                            let stage = active_tab().clone();
                                            let mut error = error;
                                            let mut loading = loading;
                                            let mut prompts = prompts;
                                            move |_| {
                                                let active_id = active_id.clone();
                                                let stage = stage.clone();
                                                spawn(async move {
                                                    *error.write() = None;
                                                    match AiService::activate_prompt(&active_id, &stage).await {
                                                        Ok(_) => {
                                                            *loading.write() = true;
                                                            match AiService::list_prompts(Some(&stage)).await {
                                                                Ok(loaded_prompts) => {
                                                                    *prompts.write() = loaded_prompts;
                                                                }
                                                                Err(e) => {
                                                                    *error.write() = Some(format!("Failed to reload prompts: {:?}", e));
                                                                }
                                                            }
                                                            *loading.write() = false;
                                                        }
                                                        Err(e) => {
                                                            *error.write() = Some(format!("Failed to activate prompt: {:?}", e));
                                                        }
                                                    }
                                                });
                                            }
                                        },
                                        "Activate Current Version"
                                    }
                                }
                            }
                        }
                    }

                    // Version history
                    div {
                        class: "p-5 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700",
                        h3 {
                            class: "text-lg font-medium text-gray-900 dark:text-gray-100 mb-4",
                            "Version History"
                        }
                        div {
                            class: "space-y-2",
                            for prompt in prompts_list.iter() {
                                div {
                                    class: "flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-900 rounded-md",
                                    div {
                                        div {
                                            class: "font-medium text-gray-900 dark:text-gray-100",
                                            {prompt.name.clone()}
                                        }
                                        div {
                                            class: "text-sm text-gray-500 dark:text-gray-400",
                                            {prompt.created_at.clone()}
                                        }
                                    }
                                    if prompt.is_active {
                                        span {
                                            class: "px-2 py-1 text-xs font-medium bg-green-100 dark:bg-green-900/20 text-green-800 dark:text-green-200 rounded",
                                            "Active"
                                        }
                                    } else {
                                        button {
                                            class: "px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700",
                                            onclick: {
                                                let prompt_id = prompt.id.clone();
                                                let stage = active_tab().clone();
                                                let mut error = error;
                                                let mut loading = loading;
                                                let mut prompts = prompts;
                                                move |_| {
                                                    let prompt_id = prompt_id.clone();
                                                    let stage = stage.clone();
                                                    spawn(async move {
                                                        *error.write() = None;
                                                        match AiService::activate_prompt(&prompt_id, &stage).await {
                                                            Ok(_) => {
                                                                *loading.write() = true;
                                                                match AiService::list_prompts(Some(&stage)).await {
                                                                    Ok(loaded_prompts) => {
                                                                        *prompts.write() = loaded_prompts;
                                                                    }
                                                                    Err(e) => {
                                                                        *error.write() = Some(format!("Failed to reload prompts: {:?}", e));
                                                                    }
                                                                }
                                                                *loading.write() = false;
                                                            }
                                                            Err(e) => {
                                                                *error.write() = Some(format!("Failed to activate prompt: {:?}", e));
                                                            }
                                                        }
                                                    });
                                                }
                                            },
                                            "Activate"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TabButton(label: String, stage: String, active: bool, onclick: EventHandler) -> Element {
    let border_class = if active {
        "border-blue-500 text-blue-600 dark:text-blue-400"
    } else {
        "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300 dark:hover:border-gray-600"
    };

    rsx! {
        button {
            class: "whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium {border_class}",
            onclick: move |_| onclick.call(()),
            {label}
        }
    }
}
