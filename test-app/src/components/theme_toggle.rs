use gloo_storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
pub fn ThemeToggle() -> Html {
    let dark = use_state(is_dark_mode);
    let is_dark = *dark;

    let onchange = Callback::from(move |event: Event| {
        let e: HtmlInputElement = event.target_dyn_into().unwrap();
        let checked = e.checked();
        dark.set(checked);
        log::info!("toggle dark: {}", checked);
    });

    use_effect_with_deps(
        move |is_dark| {
            set_dark(*is_dark);
        },
        is_dark,
    );

    html! {
        <label class="switch">
            <input type={"checkbox"} checked={is_dark} {onchange} />
            <span class="slider round"></span>
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
    class_list.toggle("dark").unwrap();
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