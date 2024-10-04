use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{HomePage, ProductsPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/products")]
    Products,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<HomePage/> },
        Route::Products => html! { <ProductsPage /> },
    }
}