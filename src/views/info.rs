use crate::components::Json;
use crate::Config;
use dioxus::prelude::*;
use serde_json::Value;

const INFO_CSS: Asset = asset!("/assets/styling/info.css");

#[component]
pub fn Info() -> Element {
    let config = use_context::<Signal<Config>>();
    let relay_url = config().relay_url.as_str().to_owned();

    let stats =
        use_resource(move || async move { crate::nip86::stats(config().relay_url.as_str()).await });

    rsx! {
        document::Link { rel: "stylesheet", href: INFO_CSS}

        div {
            id: "info",
            h1 { "Info" },

            div {
                class: "relay",
                "{relay_url}"
            }

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
