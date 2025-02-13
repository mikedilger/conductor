use crate::Globals;
use crate::components::Json;
use dioxus::prelude::*;
use serde_json::{json, Value};

const INFO_CSS: Asset = asset!("/assets/styling/info.css");

#[component]
pub fn Info() -> Element {
    let mut globals = use_context::<Signal<Globals>>();

    let stats = use_resource(move || async move {
        crate::nip86::stats(
            globals().relay_url.as_str(),
            "stats",
            json!([]),
        ).await
    });

    rsx! {
        document::Link { rel: "stylesheet", href: INFO_CSS}

        div {
            id: "info",
            h1 { "Info" },

            div {
                id: "stats",
                h2 { "Stats" },
                if stats().is_some() {
                    Json {
                        property: Value::Object(stats().unwrap()),
                    }
                } else {
                    "Loading..."
                }
            }
        }
    }
}
