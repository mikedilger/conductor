use crate::Config;
use dioxus::prelude::*;

const REPORTS_CSS: Asset = asset!("/assets/styling/reports.css");

#[component]
pub fn Reports() -> Element {
    let config = use_context::<Signal<Config>>();

    rsx! {
        document::Link { rel: "stylesheet", href: REPORTS_CSS}

        div {
            id: "reports",
            h1 { "Reports" },

            "Abuse report handling is not yet supported."
        }
    }
}
