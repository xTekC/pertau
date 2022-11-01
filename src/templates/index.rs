use perseus::{spawn_local_scoped, Template};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::prelude::{create_signal, view, Html, Scope, SsrNode, View};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Deserialize, Serialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[perseus::template_rx]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());
    let greet_msg = create_signal(cx, String::new());

    let greet = move |_| {
        spawn_local_scoped(cx, async move {
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg =
                invoke("greet", to_value(&GreetArgs { name: &name.get() }).unwrap()).await;

            log(&new_msg.as_string().unwrap());

            greet_msg.set(new_msg.as_string().unwrap());
        })
    };

    view! { cx,
        main(class="container") {
            div(class="row") {
                a(href="https://tauri.app",target="_blank") {
                    img(src="public/tauri.svg",class="logo tauri",alt="Tauri logo")
                }
                a(href="https://framesurge.sh/perseus/en-US/docs",target="_blank") {
                    span(class="logo perseus") {
                        "Perseus"
                    }
                }
            }
            p {
                "Click on the Tauri and Perseus logos to learn more."
            }
            p {
                "Recommended setup: "
                a(href="https://helix-editor.com/",target="_blank") {
                    "Helix Editor"
                }
                " + "
                a(href="https://github.com/rust-lang/rust-analyzer",target="_blank") {
                    "rust-analyzer"
                }
            }
            div(class="row") {
                input(id="greet-input",bind:value=name,placeholder="Enter a name...")
                button(type="button",on:click=greet) {
                    "Greet"
                }
            }
            p {
                b {
                    (greet_msg.get())
                }
            }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "PerTau" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
