use crate::components::{Button, RenderedEvent, UserLine};
use crate::Context;
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
    let mut context: Context = use_context::<Context>();

    let mut reload_trick = use_signal(|| 0);

    let mut tab = use_signal(|| Tab::BannedEvents);

    let events = use_resource(move || async move {
        match tab() {
            Tab::BannedEvents => {
                crate::nip86::fetchbannedevents(
                    context.config.read().relay_url.as_str(),
                    reload_trick(),
                )
                .await
            }
            Tab::AllowedEvents => {
                crate::nip86::listallowedevents(
                    context.config.read().relay_url.as_str(),
                    reload_trick(),
                )
                .await
            }
            _ => Ok(vec![]),
        }
    });

    let pubkeys = use_resource(move || async move {
        match tab() {
            Tab::BannedUsers => {
                crate::nip86::listbannedpubkeys(
                    context.config.read().relay_url.as_str(),
                    reload_trick(),
                )
                .await
            }
            Tab::AllowedUsers => {
                crate::nip86::listallowedpubkeys(
                    context.config.read().relay_url.as_str(),
                    reload_trick(),
                )
                .await
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
                onclick: move |_event: Event<MouseData>| {
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
                onclick: move |_event: Event<MouseData>| {
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
                onclick: move |_event: Event<MouseData>| {
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
                onclick: move |_event: Event<MouseData>| {
                    tab.set(Tab::AllowedUsers);
                },
                "Allowed Users"
            }
        }

        hr {
        }

        match tab() {
            Tab::BannedEvents => {
                rsx! {
                    match &*events.read_unchecked() {
                        Some(Ok(v)) => rsx! {
                            for e in v.iter().cloned() {
                                RenderedEvent {
                                    e: e.clone(),
                                    relay_url: context.config.read().relay_url.clone(),
                                }
                                div {
                                    Button {
                                        text: "Allow",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation(); // just the button, no deeper
                                            let eventid = e.id;
                                            spawn(async move {
                                                if let Err(e) = crate::nip86::allow_event(context.config.read().relay_url.as_str(), eventid).await {
                                                    context.errors.write().push(format!("{e}"));
                                                }
                                                reload_trick += 1;
                                            });
                                        },
                                        class: "moderate milddanger",
                                    }
                                    Button {
                                        text: "Return to Queue",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation(); // just the button, no deeper
                                            let eventid = e.id;
                                            spawn(async move {
                                                if let Err(e) = crate::nip86::clear_event(context.config.read().relay_url.as_str(), eventid).await {
                                                    context.errors.write().push(format!("{e}"));
                                                }
                                                reload_trick += 1;
                                            });
                                        },
                                        class: "moderate milddanger",
                                    }
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
            Tab::AllowedEvents => {
                rsx! {
                    match &*events.read_unchecked() {
                        Some(Ok(v)) => rsx! {
                            for e in v.iter().cloned() {
                                RenderedEvent {
                                    e: e.clone(),
                                    relay_url: context.config.read().relay_url.clone(),
                                }
                                div {
                                    Button {
                                        text: "Ban",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation(); // just the button, no deeper
                                            let eventid = e.id;
                                            spawn(async move {
                                                if let Err(e) = crate::nip86::ban_event(context.config.read().relay_url.as_str(), eventid).await {
                                                    context.errors.write().push(format!("{e}"));
                                                }
                                                reload_trick += 1;
                                            });
                                        },
                                        class: "moderate milddanger",
                                    }
                                    Button {
                                        text: "Return to Queue",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation(); // just the button, no deeper
                                            let eventid = e.id;
                                            spawn(async move {
                                                if let Err(e) = crate::nip86::clear_event(context.config.read().relay_url.as_str(), eventid).await {
                                                    context.errors.write().push(format!("{e}"));
                                                }
                                                reload_trick += 1;
                                            });
                                        },
                                        class: "moderate milddanger",
                                    }
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
            Tab::BannedUsers  => {
                rsx! {
                    match &*pubkeys.read_unchecked() {
                        Some(Ok(v)) => rsx! {
                            for pk in v.iter().cloned() {
                                UserLine {
                                    pk: pk
                                }
                                div {
                                    Button {
                                        text: "Clear ban",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation(); // just the button, no deeper
                                            spawn(async move {
                                                if let Err(e) = crate::nip86::clear_pubkey(context.config.read().relay_url.as_str(), pk).await {
                                                    context.errors.write().push(format!("{e}"));
                                                }
                                                reload_trick += 1;
                                            });
                                        },
                                        class: "moderate milddanger",
                                    }
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
            Tab::AllowedUsers => {
                rsx! {
                    match &*pubkeys.read_unchecked() {
                        Some(Ok(v)) => rsx! {
                            for pk in v.iter().cloned() {
                                UserLine {
                                    pk: pk
                                }
                                div {
                                    Button {
                                        text: "Clear allowance",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation(); // just the button, no deeper
                                            spawn(async move {
                                                if let Err(e) = crate::nip86::clear_pubkey(context.config.read().relay_url.as_str(), pk).await {
                                                    context.errors.write().push(format!("{e}"));
                                                }
                                                reload_trick += 1;
                                            });
                                        },
                                        class: "moderate milddanger",
                                    }
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
        }
    }
}

// Use these functions on this page in buttons:

// allow_event
// clear_event

// allow_pubkey
// clear_pubkey
