use dioxus::prelude::*;

const SETUP_CSS: Asset = asset!("/assets/styling/setup.css");

#[component]
pub fn Setup() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SETUP_CSS}

        div {
            id: "setup",
            "setup will go here"
        }
    }
}
