use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
mod components;
pub use components::{Gallery, Header};

#[function_component]
fn App() -> Html {
    let query = use_state(|| String::new());

    let oninput = {
        let query = query.clone();
        Callback::from(move |event: InputEvent| {
            query.set(event.target_dyn_into::<HtmlInputElement>().unwrap().value())
        })
    };

    html! {
        <>
            <Header />
            <div class="search-container">
                <div class="search-input">
                    <input type="text" placeholder="Search..." {oninput}/>
                    <Icon
                        class="search-icon"
                        icon_id={IconId::BootstrapSearch}
                        width={"20"}
                        height={"20"}
                    />
                </div>
           </div>

           <>
                <div>{"Hover/click to get the feature flag/"}
                    <pre style="display: inline;">{"IconId"}</pre>
                </div>

                <Gallery query={query.deref().clone()}/>
           </>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
