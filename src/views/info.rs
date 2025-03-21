use crate::components::Json;
use crate::Context;
use dioxus::prelude::*;
use serde_json::Value;

const INFO_CSS: Asset = asset!("/assets/styling/info.css");

#[component]
pub fn Info() -> Element {
    let context = use_context::<Context>();

    let stats = use_resource(move || async move {
        crate::nip86::stats(context.config.read().relay_url.as_str()).await
    });

    rsx! {
        document::Link { rel: "stylesheet", href: INFO_CSS}

        div {
            id: "info",
            h1 { "Info" },

            div {
                id: "stats",
                h2 { "Stats" },
                match &*stats.read_unchecked() {
                    Some(Ok(v)) => rsx! {
                        Json {
                            property: Value::Object(v.clone()),
                        }
                    },
                    Some(Err(e)) => rsx! {
                        "Loading failed: {e}"
                    },
                    None => rsx! {
                        "Loading..."
                    }
                }
            }
        }
    }
}
