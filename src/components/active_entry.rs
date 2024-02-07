use crate::utils::tauri_commands::invoke;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use yew::{function_component, html, Html};

#[function_component(ActiveEntry)]
pub fn active_entry() -> Html {
    let clicktest = Callback::from(|_| {
        spawn_local(async {
            // Invoke the Tauri command without arguments
            let result = invoke("clicktest", JsValue::NULL).await;
            // Directly log the result, assuming it's the expected string
            console::log_1(&result);
        });
    });

    let copy_icon = html! {
        <svg class="text-shark-50 fill-current w-8 h-8 mx-8" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
        </svg>
    };

    html! {
        <>
        <div class="flex-1 flex items-center justify-center">
            <div class="bg-shark-950 rounded-2xl">
                <div><h1 class="text-center mb-12 text-2xl">{"Reddit"}</h1></div>

                <div class="border-b border-opacity-30 border-shark-50 p-4 cursor-pointer flex justify-between" onclick={&clicktest}>
                    <div>{"Email:"}</div>
                    <div>{"testman@test.com"}</div>
                    <div>{ copy_icon.clone() }</div>
                </div>
                <div class="border-b border-opacity-30 border-shark-50 p-4 cursor-pointer flex justify-between" onclick={&clicktest}>
                    <div>{"Username: "}</div>
                    <div>{"Testman"}</div>
                    <div>{ copy_icon.clone() }</div>
                </div>
                <div class="p-4 cursor-pointer flex justify-between" onclick={&clicktest}>
                    <div>{"Password:"}</div>
                    <div>{" *********"}</div>
                    <div>{ copy_icon.clone() }</div>
                </div>
            </div>
        </div>
        </>
    }
}
