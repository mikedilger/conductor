use dioxus::prelude::*;

const REPORTS_CSS: Asset = asset!("/assets/styling/reports.css");

#[component]
pub fn Reports() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: REPORTS_CSS}

        div {
            id: "reports",
            "Abuse report handling is not yet supported."
        }
    }
}
