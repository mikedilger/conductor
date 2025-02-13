use dioxus::prelude::*;

const QUEUE_CSS: Asset = asset!("/assets/styling/queue.css");

#[component]
pub fn Queue() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: QUEUE_CSS}

        div {
            id: "queue",
            "Moderation queue support is not yet supported."
        }
    }
}
