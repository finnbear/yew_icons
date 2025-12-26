mod components;

pub use components::{Gallery, Header, ScrollToTop};
use std::ops::Deref;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

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
                        data={IconData::BOOTSTRAP_SEARCH}
                        width={"20"}
                        height={"20"}
                    />
                </div>
           </div>

           <>
                <div style="padding: 10px;">
                    <div class="help-text">
                        {"Hover/click to get the feature flag/IconData"}
                    </div>
                </div>

                <Gallery query={query.deref().clone()}/>
                <ScrollToTop />
           </>
        </>
    }
}

fn main() {
    let document = window().unwrap().document().unwrap();
    let root = document.get_element_by_id("root").unwrap();
    yew::Renderer::<App>::with_root(root).render();
}
