use yew::{function_component, html, Html};

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    html! {
        <div class="h-16 mb-8 flex flex-col justify-end items-center border border-black">
            <div class="w-24 pb-4">
                <input type="text" name="search" id="search" placeholder="search" class="border border-black" />
            </div>
        </div>
    }
}
