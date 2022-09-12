use yew::{Callback, function_component, html, use_state};


#[function_component(UseState)]
fn state() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };


    html! {
        <div>
            <p>
                <b {onclick}>{ "Current value: " }{*counter}</b>
            </p>
        </div>
    }
}

fn main() {
    yew::start_app::<UseState>();
}
