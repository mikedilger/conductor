use crate::Config;
use dioxus::prelude::*;

const USERS_CSS: Asset = asset!("/assets/styling/users.css");

#[component]
pub fn Users() -> Element {
    let config = use_context::<Signal<Config>>();
    let relay_url = config().relay_url.as_str().to_owned();

    rsx! {

        document::Link { rel: "stylesheet", href: USERS_CSS}

        div {
            id: "users",
            h1 { "Users" },

            div {
                class: "relay",
                "{relay_url}"
            }

            "User management is not yet supported."
        }
    }
}
