use dioxus::prelude::*;
use dioxus_sdk::storage::*;

const SETUP_CSS: Asset = asset!("/assets/styling/setup.css");

#[component]
pub fn Setup() -> Element {
    let mut relay_url = use_synced_storage::<LocalStorage, String>(
        "relay_url".to_string(),
        || "".to_string()
    );

    rsx! {
        document::Link { rel: "stylesheet", href: SETUP_CSS}

        div {
            id: "setup",
            "Relay Url: ",
            input {
                size: 100,
                value: "{relay_url}",
                oninput: move |event| relay_url.set(event.value())
            }
        }
    }
}
