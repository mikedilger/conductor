use dioxus::prelude::*;
use nostr::nips::nip07::BrowserSigner;
use nostr::types::url::RelayUrl;
use crate::Globals;

const SETUP_CSS: Asset = asset!("/assets/styling/setup.css");

#[component]
pub fn Setup() -> Element {
    let mut globals = use_context::<Signal<Globals>>();

    let browser_signer = BrowserSigner::new();
    let found_signer = browser_signer.is_ok();
    let url_is_ok = RelayUrl::parse(globals().relay_url.as_str()).is_ok();

    rsx! {
        document::Link { rel: "stylesheet", href: SETUP_CSS}

        div {
            id: "setup",

            div {
                class: "paragraph",

                if url_is_ok && found_signer {
                    "Setup is ",
                    span {
                        class: "success",
                        "OK"
                    }
                } else if found_signer {
                    span {
                        class: "failure",
                        "Enter a valid relay url below"
                    }
                } else if url_is_ok {
                    span {
                        class: "failure",
                        "Setup a NIP-07 browser signer"
                    }
                } else {
                    span {
                        class: "failure",
                        "Nothing is setup!"
                    }
                }
            }

            div {
                class: "paragraph",
                "Relay Url: ",
                input {
                    size: 100,
                    value: "{globals().relay_url}",
                    oninput: move |event| {
                        globals.write().relay_url = event.value();
                    },
                }
            }

            div {
                class: "paragraph",
                "Signer: ",
                if found_signer {
                    "Found"
                } else {
                    "Signer: NOT FOUND (a NIP-07 browser signer is required)"
                }
            }
        }
    }
}
