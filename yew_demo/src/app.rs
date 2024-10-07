use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Route, switch};

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}