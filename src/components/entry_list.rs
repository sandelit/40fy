use yew::{function_component, html, Html};

#[function_component(EntryList)]
pub fn entry_list() -> Html {
    html! {
        <div class="flex flex-1 flex-col bg-shark-950 h-1/4">
            <div>{"filterbar"}</div>
            <div class="bg-shark-900">{"R"}</div>
            <div class="bg-midnight text-tahiti">{"Reddit"}</div>
        </div>
    }
}
