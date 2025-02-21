use dioxus::prelude::*;
use nostr::event::Event;
use serde_json::Value;

#[component]
pub fn RenderedEvent(e: Event) -> Element {
    rsx! {
        "EVENT: ",
        "{e.content}"
    }
}
