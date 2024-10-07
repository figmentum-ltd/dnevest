use yew::prelude::*;

use crate::components::DatePicker;

#[function_component(OrderPage)]
pub fn order_page() -> Html {
    html! {
        <div class="order-page">
            <h1>{ "Choose the date of the newspaper" }</h1>
            <DatePicker />
        </div>
    }
}