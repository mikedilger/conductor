use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use serde::{Serialize, Deserialize};
use web_sys::window;

use components::Navbar;
use views::{Home, Moderate, Users, Setup};

mod components;
mod views;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct AppState {
    relay_url: String,
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/moderate")]
    Moderate,
    #[route("/users")]
    Users,
    #[route("/setup")]
    Setup,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus_sdk::storage::set_dir!();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // use_persistent_context_provider(cx);

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> { }
    }
}
