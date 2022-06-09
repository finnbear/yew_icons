#![feature(stmt_expr_attributes)]

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

            html_nested! {
                <Icon
                    {title}
                    {icon_id}
                    width={"24"}
                    height={"24".to_string()}
                    onclick={Callback::from(move |_| log::info!("{:?} click", icon_id))}
                    oncontextmenu={Callback::from(move |_| log::info!("{:?} contextmenu", icon_id))}
                    style={format!("margin: 0.1em; display: {};", display)}
                />
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
                {" - Hover to get the feature flag/"}
                <pre style="display: inline;">{"IconId"}</pre>
            </p>
            <input type="text" placeholder="Search" {oninput}/>
            <div style="display: none;">
                <Icon icon_id={IconId::FeatherArrowLeftCircle}/>
                <Icon icon_id={IconId::FeatherArrowUpCircle} width={"2em".to_owned()} height={"2em".to_owned()}/>
                <Icon icon_id={IconId::FeatherArrowRightCircle} onclick={Callback::from(|_: MouseEvent| {})}/>
            </div>
            <div style="color: black; background-color: white; padding: 0.5em;">
                <h2 style={"font-family: sans-serif; margin-top: 0;"}>{"Black on White"}</h2>
                <Gallery query={query.deref().clone()}/>
            </div>
            <div style="color: white; background-color: black; padding: 0.5em;">
                <h2 style={"font-family: sans-serif; margin-top: 0;"}>{"White on Black"}</h2>
                <Gallery query={query.deref().clone()}/>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
