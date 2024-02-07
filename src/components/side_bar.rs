use yew::{function_component, html, Html};

#[function_component(SideBar)]
pub fn side_bar() -> Html {

    let tags = ["test", "customtag", "random", "stuff"];

    html! {
        <div class="flex-1 pl-6 bg-red">
            <ul class="pl-6">
                <li>{ "All items" }</li>
                <li>{ "Favourites" }</li>
            </ul>
            <p>{ "Categories" }</p>
            <ul class="pl-6">
                <li>{ "All items" }</li>
                <li>{ "Favourites" }</li>
            </ul>
            <p>{ "Tags" }</p>

            <ul class="pl-6">
            {
                tags.into_iter().map(|tag| {
                    html!{<li key={tag}>{ tag }</li>}
                }).collect::<Html>()
            }
            </ul>
        </div>
    }
}
