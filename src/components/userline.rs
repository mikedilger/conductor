use dioxus::prelude::*;
use nostr::nips::nip01::Metadata;

#[component]
pub fn UserLine(m: Option<Option<Metadata>>) -> Element {
    rsx! {
        match m {
            Some(Some(metadata)) => {
                match metadata.name {
                    Some(ref name) => rsx! {
                        span { "{name}" }
                    },
                    None => rsx! {
                        span { "<noname>" }
                    },
                }
            },
            Some(None) => rsx! {
                span { "<nometadata>" }
            },
            None => rsx! {
                span { "LOADING" }
            }
        }
    }
}
