use crate::Globals;
use dioxus::prelude::*;

const REPORTS_CSS: Asset = asset!("/assets/styling/reports.css");

#[component]
pub fn Reports() -> Element {
    let globals = use_context::<Signal<Globals>>();
    let relay_url = globals().relay_url.as_str().to_owned();

    rsx! {
        document::Link { rel: "stylesheet", href: REPORTS_CSS}

        div {
            id: "reports",
            h1 { "Reports" },

            div {
                class: "relay",
                "{relay_url}"
            }

            "Abuse report handling is not yet supported."
        }
    }
}
