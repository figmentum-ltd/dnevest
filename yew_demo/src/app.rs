use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Route, switch};

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <nav>
                <ul>
                    <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Products}>{ "Products" }</Link<Route>></li>
                </ul>
            </nav>

            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}