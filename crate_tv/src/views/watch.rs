use dioxus::prelude::*;

use crate::components::Video;

const WATCH_CSS: Asset = asset!("/assets/styling/watch.css");

#[component]
pub fn Watch(id: String) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: WATCH_CSS }

        div { id: "watch",

            // Content
            h1 { "This is {id}'s stream!" }
            p { "This is currently not implemented." }
        }

        Video {}
    }
}
