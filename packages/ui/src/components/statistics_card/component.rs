use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StatisticsCardProps {
    pub icon: Element,
    pub value: usize,
    pub label: String,
    #[props(default = "text-gray-900 dark:text-white".to_string())]
    pub value_color: String,
}

#[component]
pub fn StatisticsCard(props: StatisticsCardProps) -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 overflow-hidden shadow rounded-lg",
            div {
                class: "p-5",
                div {
                    class: "flex flex-col",
                    div {
                        class: "flex items-center justify-between",
                        div {
                            class: "{props.value_color}",
                            {props.icon}
                        }
                        div {
                            class: "flex flex-col items-center flex-1",
                            div {
                                class: "text-3xl font-semibold leading-none {props.value_color}",
                                "{props.value}"
                            }
                            p {
                                class: "mt-2 text-sm font-medium text-gray-500 dark:text-gray-400",
                                {props.label}
                            }
                        }
                    }
                }
            }
        }
    }
}
