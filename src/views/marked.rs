use crate::components::RenderedEvent;
use crate::Config;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    BannedEvents,
    AllowedEvents,
    BannedUsers,
    AllowedUsers,
}

#[component]
pub fn Marked() -> Element {
    let config = use_context::<Signal<Config>>();
    let relay_url = config().relay_url.as_str().to_owned();

    let mut reload_trick = use_signal(|| 0);

    let mut tab = use_signal(|| Tab::BannedEvents);

    let events = use_resource(move || async move {
        match tab() {
            Tab::BannedEvents => {
                crate::nip86::listbannedevents(config().relay_url.as_str(), reload_trick()).await
            }
            Tab::AllowedEvents => {
                crate::nip86::listallowedevents(config().relay_url.as_str(), reload_trick()).await
            }
            _ => Ok(vec![]),
        }
    });

    let pubkeys = use_resource(move || async move {
        match tab() {
            Tab::BannedUsers => {
                crate::nip86::listbannedpubkeys(config().relay_url.as_str(), reload_trick()).await
            }
            Tab::AllowedUsers => {
                crate::nip86::listallowedpubkeys(config().relay_url.as_str(), reload_trick()).await
            }
            _ => Ok(vec![]),
        }
    });

    rsx! {
        if tab() == Tab::BannedEvents {
            span {
                class: "tabs selected",
                "Banned Events"
            }
        } else {
            span {
                class: "tabs",
                onclick: move |event: Event<MouseData>| {
                    tab.set(Tab::BannedEvents);
                },
                "Banned Events",
            }
        }

        span { "|" }

        if tab() == Tab::AllowedEvents {
            span {
                class: "tabs selected",
                "Allowed Events"
            }
        } else {
            span {
                class: "tabs",
                onclick: move |event: Event<MouseData>| {
                    tab.set(Tab::AllowedEvents);
                },
                "Allowed Events"
            }
        }

        span { "|" }

        if tab() == Tab::BannedUsers {
            span {
                class: "tabs selected",
                "Banned Users"
            }
        } else {
            span {
                class: "tabs",
                onclick: move |event: Event<MouseData>| {
                    tab.set(Tab::BannedUsers);
                },
                "Banned Users"
            }
        }

        span { "|" }

        if tab() == Tab::AllowedUsers {
            span {
                class: "tabs selected",
                "Allowed Users"
            }
        } else {
            span {
                class: "tabs",
                onclick: move |event: Event<MouseData>| {
                    tab.set(Tab::AllowedUsers);
                },
                "Allowed Users"
            }
        }

        hr {
        }

        match tab() {
            Tab::BannedEvents | Tab::AllowedEvents => {
                rsx! {
                    match &*events.read_unchecked() {
                        Some(Ok(v)) => rsx! {
                            for e in v.iter().cloned() {
                                RenderedEvent {
                                    e: e.clone(),
                                    relay_url: relay_url.clone(),
                                }
                            }
                            div { "end." }
                        },
                        Some(Err(e)) => rsx! {
                            "Loading failed: {e}"
                        },
                        None => rsx! {
                            "Loading..."
                        }
                    }
                }
            },
            Tab::BannedUsers | Tab::AllowedUsers => {
                rsx! {
                    match &*pubkeys.read_unchecked() {
                        Some(Ok(v)) => rsx! {
                            for pk in v.iter().cloned() {
                                div { "{pk}" }
                            }
                            div { "end." }
                        },
                        Some(Err(e)) => rsx! {
                            "Loading failed: {e}"
                        },
                        None => rsx! {
                            "Loading..."
                        }
                    }
                }
            },
        }
    }
}

// Use these functions on this page in buttons:

// allow_event
// clear_event

// allow_pubkey
// clear_pubkey
