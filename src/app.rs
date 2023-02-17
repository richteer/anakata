use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GetHtmlArgs {
}

#[function_component(App)]
pub fn app() -> Html {
    
    let html_test = use_state(|| String::new());
    {
        let html_test = html_test.clone();
        let html_test2 = html_test.clone();
        use_mount(
            move || {
                spawn_local(async move {
                    let html = invoke("gethtml", to_value(&GetHtmlArgs {}).unwrap()).await;
                    html_test.set(html.as_string().unwrap());
                });
            }
        );

        // TODO: Don't use a dang interval, figure out how to send data from Tauri instead
        use_interval(
            move || {
                let html_test = html_test2.clone();
                spawn_local(async move {
                    let html = invoke("gethtml", to_value(&GetHtmlArgs {}).unwrap()).await;
                    html_test.set(html.as_string().unwrap());
                });
            },
            5 * 1000
        );
    }

    let parsed = Html::from_html_unchecked(AttrValue::from((*html_test).clone()));
    
    html! {
        <main class="container">
            {parsed}
        </main>
    }
}
