use gloo_storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[function_component]
pub fn ThemeToggle() -> Html {
    let dark = use_state(is_dark_mode);
    let is_dark = *dark;

    let onchange = Callback::from(move |event: Event| {
        let e: HtmlInputElement = event.target_dyn_into().unwrap();
        let checked = e.checked();
        dark.set(checked);
    });

    use_effect_with(is_dark, move |is_dark| {
        set_dark(*is_dark);
    });

    html! {
        <label class="switch">
            <input type={"checkbox"} checked={is_dark} {onchange} />
            <span class="slider">
                <Icon class="icon" data={if is_dark { IconData::BOOTSTRAP_MOON_FILL } else { IconData::BOOTSTRAP_SUN_FILL }} width={"26"} height={"26"} />
            </span>
        </label>
    }
}

fn set_dark(is_dark: bool) {
    LocalStorage::set("dark", is_dark).unwrap();
    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();

    let class_list = body.class_list();

    if is_dark {
        class_list.add_1("dark").unwrap();
    } else {
        class_list.remove_1("dark").unwrap();
    }
}

fn is_dark_mode() -> bool {
    LocalStorage::get::<bool>("dark").unwrap_or_else(|_| {
        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();

        let class_list = body.class_list();
        class_list.contains("dark")
    })
}
