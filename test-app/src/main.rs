use enum_iterator::IntoEnumIterator;
use yew::prelude::*;
use yew_icons::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Icon icon_id={IconId::FeatherArrowLeftCircle}/>
            <Icon icon_id={IconId::FeatherArrowUpCircle} width={"2em".to_owned()} height={"2em".to_owned()}/>
            <Icon icon_id={IconId::FeatherArrowRightCircle} onclick={Callback::from(|_: MouseEvent| {})}/>
            <div style="color: black; background-color: white;">
                { IconId::into_enum_iter().map(|icon_id| html_nested!(<Icon {icon_id}/>)).collect::<Html>() }
            </div>
            <div style="color: white; background-color: black;">
                { IconId::into_enum_iter().map(|icon_id| html_nested!(<Icon {icon_id}/>)).collect::<Html>() }
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
