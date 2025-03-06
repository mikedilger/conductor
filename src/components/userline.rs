use crate::Context;
use dioxus::prelude::*;
use nostr::key::public_key::PublicKey;

#[component]
pub fn UserLine(pk: PublicKey) -> Element {
    let mut context: Context = use_context::<Context>();

    let metadata = use_resource(move || async move {
        match crate::nostr::get_metadata(
            pk,
            context.config.read().discovery_relay_url.clone()
        ).await {
            Err(e) => {
                context.errors.write().push(format!("{e}"));
                None
            },
            Ok(opt) => opt
        }
    });

    let npub = nostr::nips::nip21::Nip21::Pubkey(pk)
        .to_nostr_uri()
        .unwrap();

    rsx! {
        span {
            "name: "
        }

        match metadata() {
            Some(Some(metadata)) => {
                match metadata.name {
                    Some(ref name) => rsx! {
                        span {
                            class: "name",
                            "{name}"
                        },
                    },
                    None => rsx! {
                        span {
                            class: "placeholder",
                            "<noname>"
                        }
                    },
                }
            },
            Some(None) => rsx! {
                span {
                    class: "placeholder",
                    "<nometadata>"
                }
            },
            None => rsx! {
                span {
                    class: "placeholder",
                    "LOADING"
                }
            }
        }

        div {
            class: "indent",
            "pubkey hex: ",
            span {
                class: "pubkey",
                "{pk}"
            }
        }

        div {
            class: "indent",
            span {
                class: "pubkey",
                "{npub}"
            }
        }
    }
}
