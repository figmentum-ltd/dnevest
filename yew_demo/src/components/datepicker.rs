use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::events::InputEvent;

#[function_component(DatePicker)]
pub fn date_picker() -> Html {
    let selected_date = use_state(|| "".to_string());

    let oninput = {
        let selected_date = selected_date.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            selected_date.set(input.value());
        })
    };

    let format_date = |date_str: &str| {
        if date_str.len() == 10 { 
            let parts: Vec<&str> = date_str.split('-').collect();
            if parts.len() == 3 {
                format!("{}/{}/{}", parts[2], parts[1], parts[0])
            } else {
                date_str.to_string() 
            }
        } else {
            date_str.to_string() 
        }
    };

    html! {
        <div>
            <label for="date-input">{ "Select Date: " }</label>
            <input
                id="date-input"
                type="date"
                value={(*selected_date).clone()}
                oninput={oninput}
            />
            <p>{ format!("Selected date: {}", format_date(&selected_date))  }</p>
        </div>
    }
}
