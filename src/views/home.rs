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

        blockquote {

            div {
                class: "paragraph",
                "Step 1: Navigate to ",
                a {
                    href: "/setup",
                    "setup"
                }
                " to pick a relay and configure your identity"
            }

            div {
                class: "paragraph",
                "Step 2: Navigate to ",
                a {
                    href: "/moderate",
                    "moderate"
                }
                " moderate to approve or reject events in the moderation queue"
            }

            div {
                class: "paragraph",
                "Step 3: Navigate to ",
                a {
                    href: "/users",
                    "users"
                }
                " users to change users and moderators"
            }

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
