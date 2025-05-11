use dioxus::prelude::*;

#[component]
pub fn Video() -> Element {
    rsx! {
        div { id: "video",
            p { "This is a video" }
        }
    }
}
