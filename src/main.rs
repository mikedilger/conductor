mod components;
mod nip86;
mod nostr;
mod views;

use components::Navbar;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use serde::{Deserialize, Serialize};
use views::{Help, Home, Info, Marked, Queue, Reports, Setup, Users};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub relay_url: String,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub config: Signal<Config>,
    pub errors: Signal<Vec<String>>,
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/setup")]
    Setup,
    #[route("/info")]
    Info,
    #[route("/queue")]
    Queue,
    #[route("/marked")]
    Marked,
    #[route("/reports")]
    Reports,
    #[route("/users")]
    Users,
    #[route("/help")]
    Help,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus_sdk::storage::set_dir!();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let config =
        use_synced_storage::<LocalStorage, Config>("config".to_string(), || Default::default());

    let _context = use_context_provider(|| Context {
        config,
        errors: Signal::new(vec![]),
    });

    info!("App started");

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
