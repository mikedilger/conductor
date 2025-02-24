use crate::{Config, Route};
use dioxus::prelude::*;
use nostr::nips::nip07::BrowserSigner;
use nostr::types::url::RelayUrl;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let config = use_context::<Signal<Config>>();
    let relay_url = config().relay_url.as_str().to_owned();

    let browser_signer = BrowserSigner::new();
    let found_signer = browser_signer.is_ok();
    let url_is_ok = RelayUrl::parse(config().relay_url.as_str()).is_ok();
    let setup = found_signer & url_is_ok;

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Link {
                class: "title",
                to: Route::Home {},
                "Conductor"
            }
            Link {
                to: Route::Setup {},
                "Setup"
            }
            if setup {
                Link {
                    to: Route::Info {},
                    "Info"
                }
                Link {
                    to: Route::Queue {},
                    "Queue"
                }
                Link {
                    to: Route::Marked {},
                    "Marked"
                }
                Link {
                    to: Route::Reports {},
                    "Reports"
                }
                Link {
                    to: Route::Users {},
                    "Users"
                }
            }
            Link {
                to: Route::Help {},
                "Help"
            }
        }

        div {
            class: "relay",
            "{relay_url}"
        }

        Outlet::<Route> {}
    }
}
