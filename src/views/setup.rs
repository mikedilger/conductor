use dioxus::prelude::*;
use crate::Globals;

const SETUP_CSS: Asset = asset!("/assets/styling/setup.css");

#[component]
pub fn Setup() -> Element {
    let globals = use_context::<Globals>().unwrap();

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
