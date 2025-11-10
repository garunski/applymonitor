use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::bs_icons::{BsBook, BsChatDots, BsCollection, BsGear, BsLightning, BsStars},
    Icon,
};

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {

        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/",
                    Icon {
                        class: "mr-2",
                        width: 16,
                        height: 16,
                        fill: "currentColor",
                        icon: BsBook,
                    }
                    "Learn Dioxus"
                }
                a { href: "https://dioxuslabs.com/awesome",
                    Icon {
                        class: "mr-2",
                        width: 16,
                        height: 16,
                        fill: "currentColor",
                        icon: BsLightning,
                    }
                    "Awesome Dioxus"
                }
                a { href: "https://github.com/dioxus-community/",
                    Icon {
                        class: "mr-2",
                        width: 16,
                        height: 16,
                        fill: "currentColor",
                        icon: BsCollection,
                    }
                    "Community Libraries"
                }
                a { href: "https://github.com/DioxusLabs/sdk",
                    Icon {
                        class: "mr-2",
                        width: 16,
                        height: 16,
                        fill: "currentColor",
                        icon: BsGear,
                    }
                    "Dioxus Development Kit"
                }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    Icon {
                        class: "mr-2",
                        width: 16,
                        height: 16,
                        fill: "currentColor",
                        icon: BsStars,
                    }
                    "VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM",
                    Icon {
                        class: "mr-2",
                        width: 16,
                        height: 16,
                        fill: "currentColor",
                        icon: BsChatDots,
                    }
                    "Community Discord"
                }
            }
        }
    }
}
