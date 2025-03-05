use dioxus::prelude::*;

const USERS_CSS: Asset = asset!("/assets/styling/users.css");

#[component]
pub fn Users() -> Element {
    rsx! {

        document::Link { rel: "stylesheet", href: USERS_CSS}

        div {
            id: "users",
            h1 { "Users" },

            "User management is not yet supported."
        }
    }
}
