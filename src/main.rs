use gloo_utils::document;

use slab::Slab;
use web_sys::Element;
use yew::prelude::*;

mod counter;

use counter::{CounterModel, CounterProps};

pub enum Msg {
    SpawnCounterAppInstance,
    DestroyCounterApp(usize)
}

pub struct Model {
    apps: Slab<(Element, AppHandle<CounterModel>)>,
    apps_container_ref: NodeRef
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            apps: Slab::new(),
            apps_container_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let app_container = self
            .apps_container_ref
            .cast::<Element>()
            .expect("Failed to cast app container div to HTMLElement");

        match msg {
            Msg::SpawnCounterAppInstance => {
                let app_div = document()
                    .create_element("div")
                    .expect("Failed to create <div> element");
                
                let _ = app_container
                    .append_child(&app_div)
                    .expect("Failed to append child <div> element to app container div");
                
                let app_entry = self.apps.vacant_entry();

                let app_key = app_entry.key();
                let new_counter_app = yew::start_app_with_props_in_element(app_div.clone(),
                CounterProps {
                        destroy_callback: ctx
                            .link()
                            .callback(move |_| Msg::DestroyCounterApp(app_key))
                });

                app_entry.insert((app_div, new_counter_app));
            }
            Msg::DestroyCounterApp(app_id) => {
                let (app_div, app) = self.apps.remove(app_id);

                app.destroy();

                app_div.remove()
            }
        }

        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="panel"> 
                    <button class="create" onclick={ctx.link().callback(|_| Msg::SpawnCounterAppInstance)}>
                    {"Spawn Counter App"}
                    </button>
                </div>
                <div ref={self.apps_container_ref.clone()}></div>
            </>
        }
    }
}


// #[function_component(MainApp)]
// fn state() -> Html {
//     let counter = use_state(|| 0);

//     let onclick = {
//         let counter = counter.clone();
//         Callback::from(move |_| counter.set(*counter + 1))
//     };

//     html! { <div {onclick}>
//         <h1>{"Cam Henderson"}</h1>
//         <i>{"This is a quick website to showcase my work and provide links to my other socials"}</i>
//         <p>{"If you click this div i'll increment a count"}<br/>{"The count is: "}{*counter}</p>
//         </div> }
// }

fn main() {
    yew::start_app::<Model>();
}
