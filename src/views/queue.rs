use crate::Config;
use crate::components::Json;
use dioxus::prelude::*;

const QUEUE_CSS: Asset = asset!("/assets/styling/queue.css");

#[component]
pub fn Queue() -> Element {
    let config = use_context::<Signal<Config>>();
    let relay_url = config().relay_url.as_str().to_owned();

    let mod_queue = use_resource(move || async move {
        crate::nip86::mod_queue(
            config().relay_url.as_str(),
        ).await
    });

    rsx! {
        document::Link { rel: "stylesheet", href: QUEUE_CSS}

        div {
            id: "queue",
            h1 { "Moderation Queue" }

            div {
                class: "relay",
                "{relay_url}"
            }

            match &*mod_queue.read_unchecked() {
                Some(Ok(v)) => rsx! {
                    for e in v.iter() {
                        Json {
                            property: e.clone(),
                        }
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
