use dioxus::prelude::*;

use crate::Route;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of the app since every page is under the layout
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Following {}, "Following" }
        }

        // Renders the current page under the Navbar
        Outlet::<Route> {}
    }
}
