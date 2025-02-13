mod components;
mod views;
mod nip86;

use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use components::Navbar;
use serde::{Serialize, Deserialize};
use views::{Home, Moderate, Users, Setup};


#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Globals {
    pub relay_url: String,
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/setup")]
    Setup,
    #[route("/moderate")]
    Moderate,
    #[route("/users")]
    Users,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus_sdk::storage::set_dir!();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let globals = use_synced_storage::<LocalStorage, Globals>(
        "globals".to_string(),
        || Default::default()
    );
    use_context_provider(|| globals);

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
