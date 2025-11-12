use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsCheck, BsChevronDown},
    Icon,
};
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectGroupProps, SelectListProps, SelectOptionProps, SelectProps,
    SelectTriggerProps, SelectValueProps,
};

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    rsx! {
        select::Select {
            class: "relative",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            name: props.name,
            placeholder: props.placeholder,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    rsx! {
        select::SelectTrigger {
            class: "block w-full rounded-md border-0 px-3 py-1.5 text-base text-zinc-900 shadow-sm ring-1 ring-inset ring-zinc-300 placeholder:text-zinc-400 focus:ring-2 focus:ring-inset focus:ring-zinc-900 sm:text-sm/6 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-white/5 dark:text-white dark:ring-white/10 dark:placeholder:text-zinc-500 dark:focus:ring-white flex items-center justify-between gap-x-2",
            attributes: props.attributes,
            {props.children}
            Icon {
                class: "h-5 w-5 shrink-0 text-zinc-400 dark:text-zinc-500",
                width: 20,
                height: 20,
                fill: "currentColor",
                icon: BsChevronDown,
            }
        }
    }
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    rsx! {
        select::SelectValue { attributes: props.attributes }
    }
}

#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    rsx! {
        select::SelectList {
            class: "absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm dark:bg-zinc-800 dark:ring-white/10",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    rsx! {
        select::SelectGroup {
            class: "",
            disabled: props.disabled,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    rsx! {
        select::SelectGroupLabel {
            class: "px-3 py-2 text-xs font-semibold text-zinc-500 dark:text-zinc-400",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    rsx! {
        select::SelectOption::<T> {
            class: "relative cursor-pointer select-none py-2 pl-3 pr-9 text-zinc-900 hover:bg-zinc-50 disabled:opacity-50 disabled:cursor-not-allowed dark:text-white dark:hover:bg-zinc-700",
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectItemIndicator() -> Element {
    rsx! {
        div {
            class: "absolute inset-y-0 right-0 flex items-center pr-4 text-zinc-900 dark:text-white",
            select::SelectItemIndicator {
                Icon {
                    class: "h-5 w-5",
                    width: 20,
                    height: 20,
                    fill: "currentColor",
                    icon: BsCheck,
                }
            }
        }
    }
}
