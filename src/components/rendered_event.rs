use dioxus::prelude::*;
use nostr::event::Event;
use serde_json::Value;

const EVENT_CSS: Asset = asset!("/assets/styling/event.css");

#[component]
pub fn RenderedEvent(e: Event, relay_url: String) -> Element {
    let nevent = nostr::nips::nip21::Nip21::Event(nostr::nips::nip19::Nip19Event {
        event_id: e.id,
        author: Some(e.pubkey),
        kind: Some(e.kind),
        relays: vec![relay_url],
    })
    .to_nostr_uri()
    .unwrap();

    let npub = nostr::nips::nip21::Nip21::Pubkey(e.pubkey)
        .to_nostr_uri()
        .unwrap();

    rsx! {
        document::Link { rel: "stylesheet", href: EVENT_CSS }

        div {
            class: "event",

            div {
                "Id: ",
                span {
                    class: "id",
                    "{e.id}"
                }
            }

            div {
                class: "indent",
                "{nevent}"
            }

            div {
                "Pubkey: ",
                span {
                    class: "pubkey",
                    "{e.pubkey}"
                }
            }

            div {
                class: "indent",
                span {
                    class: "pubkey",
                    "{npub}"
                }
            }

            div {
                "kind: ",
                span {
                    class: "kind",
                    "{e.kind}"
                }
            }

            div {
                "Content: ",
                span {
                    class: "content",
                    "{e.content}"
                }
            }
        }
    }
}
