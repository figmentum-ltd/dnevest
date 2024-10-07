use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{GiftPage, HomePage, OrderPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/gift")]
    Gift,
    #[at("/order")]
    Order,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<HomePage/> },
        Route::Gift => html! { <GiftPage /> },
        Route::Order => html! { <OrderPage /> },
    }
}