#![feature(stmt_expr_attributes)]

use enum_iterator::IntoEnumIterator;
use yew::prelude::*;
use yew_icons::*;

#[function_component(Gallery)]
fn gallery() -> Html {
    html! {
        IconId::into_enum_iter().map(|icon_id| html_nested!{
            <span title={format!("{:?}", icon_id)} style="margin: 0.1em;">
                <Icon {icon_id}/>
            </span>
        }).collect::<Html>()
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <div style="display: none;">
                <Icon icon_id={IconId::FeatherArrowLeftCircle}/>
                <Icon icon_id={IconId::FeatherArrowUpCircle} width={"2em".to_owned()} height={"2em".to_owned()}/>
                <Icon icon_id={IconId::FeatherArrowRightCircle} onclick={Callback::from(|_: MouseEvent| {})}/>
            </div>
            <div style="color: black; background-color: white; padding: 0.5em;">
                <h1 style={"font-family: sans-serif; margin-top: 0;"}>{"Black on White"}</h1>
                <Gallery/>
            </div>
            <div style="color: white; background-color: black; padding: 0.5em;">
                <h1 style={"font-family: sans-serif; margin-top: 0;"}>{"White on Black"}</h1>
                <Gallery/>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
