use dioxus::prelude::*;

const MODERATE_CSS: Asset = asset!("/assets/styling/moderate.css");

#[component]
pub fn Moderate() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MODERATE_CSS}

        div {
            id: "moderate",
            "moderate will go here"
        }
    }
}
