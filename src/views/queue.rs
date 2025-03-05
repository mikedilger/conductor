use crate::components::{Button, RenderedEvent};
use crate::Config;
use dioxus::prelude::*;

const QUEUE_CSS: Asset = asset!("/assets/styling/queue.css");

#[component]
pub fn Queue() -> Element {
    let config = use_context::<Signal<Config>>();
    let relay_url = config().relay_url.as_str().to_owned();

    let mut reload_trick = use_signal(|| 0);

    let mod_queue = use_resource(move || async move {
        crate::nip86::mod_queue(config().relay_url.as_str(), reload_trick()).await
    });

    rsx! {
        document::Link { rel: "stylesheet", href: QUEUE_CSS}

        div {
            id: "queue",
            h1 { "Moderation Queue" }

            match &*mod_queue.read_unchecked() {
                Some(Ok(v)) => rsx! {
                    for e in v.iter().cloned() {
                        RenderedEvent {
                            e: e.clone(),
                            relay_url: relay_url.clone(),
                        }
                        Button {
                            text: "Allow",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                let eventid = e.id;
                                spawn(async move {
                                    crate::nip86::allow_event(config().relay_url.as_str(), eventid).await;
                                    reload_trick += 1;
                                });
                            },
                            class: "moderate default",
                        }
                        Button {
                            text: "Ban (but Keep)",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                let eventid = e.id;
                                spawn(async move {
                                    crate::nip86::ban_event(config().relay_url.as_str(), eventid).await;
                                    reload_trick += 1;
                                });
                            },
                            class: "moderate milddanger",
                        }
                        Button {
                            text: "Ban and Burn!",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                let eventid = e.id;
                                spawn(async move {
                                    crate::nip86::ban_event(config().relay_url.as_str(), eventid).await;
                                    crate::nip86::remove_event(config().relay_url.as_str(), eventid).await;
                                    reload_trick += 1;
                                });
                            },
                            class: "moderate danger",
                        }
                        br {
                        }
                        Button {
                            text: "Allow User",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                let eventpk = e.pubkey;
                                spawn(async move {
                                    crate::nip86::allow_pubkey(config().relay_url.as_str(), eventpk).await;
                                    reload_trick += 1;
                                });
                            },
                            class: "moderate default",
                        }
                        Button {
                            text: "Ban User (Keep)",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                let eventpk = e.pubkey;
                                spawn(async move {
                                    crate::nip86::ban_pubkey(config().relay_url.as_str(), eventpk).await;
                                    reload_trick += 1;
                                });
                            },
                            class: "moderate milddanger",
                        }
                        Button {
                            text: "Ban User & Burn!",
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation(); // just the button, no deeper
                                let eventid = e.id;
                                let eventpk = e.pubkey;
                                spawn(async move {
                                    crate::nip86::ban_pubkey(config().relay_url.as_str(), eventpk).await;
                                    crate::nip86::remove_event(config().relay_url.as_str(), eventid).await;
                                    reload_trick += 1;
                                });
                            },
                            class: "moderate danger",
                        }
                    }
                    div {
                        "end."
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
