use dioxus::prelude::*;

const INFO_CSS: Asset = asset!("/assets/styling/info.css");

#[component]
pub fn Info() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: INFO_CSS}

        div {
            id: "info",
            "Relay Info is not yet supported"
        }
    }
}
