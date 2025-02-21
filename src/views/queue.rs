use crate::components::{Button, ButtonProps, RenderedEvent};
use crate::Config;
use dioxus::prelude::*;
use dioxus::logger::tracing::info;

const QUEUE_CSS: Asset = asset!("/assets/styling/queue.css");

#[component]
pub fn Queue() -> Element {
    let config = use_context::<Signal<Config>>();
    let relay_url = config().relay_url.as_str().to_owned();

    let mod_queue =
        use_resource(
            move || async move { crate::nip86::mod_queue(config().relay_url.as_str()).await },
        );

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
                        RenderedEvent {
                            e: e.clone(),
                            relay_url: relay_url.clone(),
                        }
                        Button {
                            text: "Approve",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                info!("Clicked Approve: {event:?}")
                            },
                            class: "default",
                        }
                        Button {
                            text: "Approve User",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                info!("Clicked Approve User: {event:?}")
                            },
                            class: "default",
                        }
                        Button {
                            text: "DELETE",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                info!("Clicked DELETE: {event:?}")
                            },
                            class: "danger",
                        }
                        Button {
                            text: "DELETE and BAN USER",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                info!("Clicked DELETE and BAN USER: {event:?}")
                            },
                            class: "danger",
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
