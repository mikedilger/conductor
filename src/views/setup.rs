use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use crate::Globals;

const SETUP_CSS: Asset = asset!("/assets/styling/setup.css");

#[component]
pub fn Setup() -> Element {
    let mut globals = use_context::<Signal<Globals>>();

    rsx! {
        document::Link { rel: "stylesheet", href: SETUP_CSS}

        div {
            id: "setup",
            "Relay Url: ",
            input {
                size: 100,
                value: "{globals().relay_url}",
                oninput: move |event| {
                    globals.write().relay_url = event.value();
                },
            }
        }
    }
}
