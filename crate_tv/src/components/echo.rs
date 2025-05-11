use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Test echo component.
#[component]
pub fn Echo() -> Element {
    // Must be run in a consistent order every time the component is rendered. (can't be put in other hooks, async blocks, if statements, loops, etc.)
    let mut response = use_signal(String::new);

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }

        div { id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                // Runs whenever input changes.
                oninput: move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            // Signals can be called like functions.
            if !response().is_empty() {
                p {
                    "Server echoed: "
                    // Whenever the signal changes, the component will rerun.
                    i { "{response}" }
                }
            }
        }
    }
}

// Public api, pretty GREEN🟩
#[server]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
