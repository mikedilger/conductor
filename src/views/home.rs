use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS}

        div {
            id: "home",
            "Conductor is a ",
            a {
                href: "https://github.com/nostr-protocol/nips",
                "nostr"
            }
            " relay manager.",
        }

        div {
            class: "paragraph",
            "conduuctor is available on ",
            a {
                href: "https://github.com/mikedilger/conductor",
                "github"
            }
        }
    }
}
