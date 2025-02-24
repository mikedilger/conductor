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

    let mut tab = use_signal(|| Tab::BannedEvents);
    // tab(), or just tab = X;

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
            Tab::BannedEvents => {
                "Banned Events TBD"
            },
            Tab::AllowedEvents => {
                "Allowed Events TBD"
            },
            Tab::BannedUsers => {
                "Banned Users TBD"
            },
            Tab::AllowedUsers => {
                "Allowed Users TBD"
            },
        }
    }
}

// Use these functions on this page:

// allow_event
// clear_event

// allow_pubkey
// clear_pubkey

// listallowedevents
// listbannedevents
// listallowedpubkeys
// listbannedpubkeys
