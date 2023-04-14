//#![feature(stmt_expr_attributes)]

use enum_iterator::IntoEnumIterator;
use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::*;

#[derive(Properties, PartialEq)]
struct GalleryProps {
    query: String,
}

#[function_component(Gallery)]
fn gallery(props: &GalleryProps) -> Html {
    html! {
        IconId::into_enum_iter().map(|icon_id| {
            let title = format!("{:?}", icon_id);

            let display = if props.query.is_empty() || props.query.to_ascii_lowercase().split(' ').all(|word| title.to_ascii_lowercase().contains(word)) {
                "initial"
            } else {
                "none"
            };

            let onclick = web_sys::window().unwrap().navigator().clipboard().map(|clipboard| Callback::from(move |_: MouseEvent| {
                log::info!("clicked {:?} {:?}", icon_id, clipboard.write_text(&format!("{:?}", icon_id)));
            }));

            html_nested! {
                <div class="icon">
                    <Icon
                    {title}
                    {icon_id}
                    width={"24"}
                    height={"24"}
                    onclick={onclick.clone()}
                    oncontextmenu={onclick}
                    style={format!("margin: 0.1em; display: {};", display)}
                />
                </div>
            }
        }).collect::<Html>()
    }
}

#[function_component(App)]
fn app() -> Html {
    let query = use_state(|| String::new());

    let oninput = {
        let query = query.clone();
        Callback::from(move |event: InputEvent| {
            query.set(event.target_dyn_into::<HtmlInputElement>().unwrap().value())
        })
    };

    html! {
        <>
            <h1 style={"font-family: sans-serif; margin-top: 0;"}>{"yew_icons"}</h1>
            <p>
                <a href={"https://github.com/finnbear/yew_icons"}>{"GitHub"}</a>
                {" - "}
                <a href={"https://crates.io/crates/yew_icons"}>{"crates.io"}</a>
                {" - Hover/click to get the feature flag/"}
                <pre style="display: inline;">{"IconId"}</pre>
            </p>
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
            <div style="display: none;">
                <Icon icon_id={IconId::FeatherArrowLeftCircle}/>
                <Icon icon_id={IconId::FeatherArrowUpCircle} width={"2em".to_owned()} height={"2em".to_owned()}/>
                <Icon icon_id={IconId::FeatherArrowRightCircle} onclick={Callback::from(|_: MouseEvent| {})}/>
            </div>
            
            <div class="gallery">
                <Gallery query={query.deref().clone()}/>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
