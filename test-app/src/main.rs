use yew::prelude::*;
use yew_icons::*;
use enum_iterator::IntoEnumIterator;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
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
