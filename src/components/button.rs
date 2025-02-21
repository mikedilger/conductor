use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use nostr::event::Event;

const BUTTON_CSS: Asset = asset!("/assets/styling/button.css");

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ButtonProps {
    pub onclick: EventHandler<MouseEvent>,
    pub text: String,
    pub class: String,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BUTTON_CSS }

        button {
            onclick: move |evt| props.onclick.call(evt),
            class: "{props.class}",
            "{props.text}"
        }
    }
}
