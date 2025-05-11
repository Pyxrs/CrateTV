use dioxus::prelude::*;

use crate::components::Echo;

#[component]
pub fn Home() -> Element {
    rsx! {
        Echo {}
    }
}
