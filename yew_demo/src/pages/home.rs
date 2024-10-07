use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let navigator = use_navigator().unwrap();

    let on_gift_click = {
        let navigator = navigator.clone();
            Callback::from(move |_| {
            navigator.push(&Route::Gift);
        })
    };
    let on_order_click ={
        let navigator = navigator.clone();
            Callback::from(move |_| {
            navigator.push(&Route::Order);
        })
    };

    html! {
        <div class="home">
            <button style="margin: 10px; padding: 10px 20px;" onclick={on_gift_click.clone()}>{ "What to gift" }</button>
            <button style="margin: 10px; padding: 10px 20px;" onclick={on_order_click.clone()}>{ "I want to order" }</button>
        </div>
    }
}