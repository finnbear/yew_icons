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
    let initial_icons = use_memo(|_| IconId::into_enum_iter().collect::<Vec<IconId>>(), ());
    let icons = use_memo(
        |query| {
            initial_icons
                .iter()
                .filter(|icon_id| {
                    let title = format!("{:?}", icon_id);
                    query
                        .to_ascii_lowercase()
                        .split(' ')
                        .all(|word| title.to_ascii_lowercase().contains(word))
                })
                .cloned()
                .collect::<Vec<IconId>>()
        },
        props.query.clone(),
    );

    html! {
        icons.iter().cloned().map(|icon_id| {
            let onclick = web_sys::window().unwrap().navigator().clipboard().map(|clipboard| Callback::from(move |_: MouseEvent| {
                log::info!("clicked {:?} {:?}", icon_id, clipboard.write_text(&format!("{:?}", icon_id)));
            }));

            let title = format!("{:?}", icon_id);
            let icon_name = title.clone();

            html_nested! {
                <div class="icon">
                    <Icon
                    {title}
                    {icon_id}
                    width={"24"}
                    height={"24"}
                    onclick={onclick.clone()}
                    oncontextmenu={onclick}
                />
                <p class="icon-name">{icon_name}</p>
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
