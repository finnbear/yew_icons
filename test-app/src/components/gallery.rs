use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub query: String,
}

#[function_component]
pub fn Gallery(props: &GalleryProps) -> Html {
    let initial_icons = use_memo((), |_| {
        IconData::ENUMERATE
            .iter()
            .copied()
            .map(|data| (format!("{data:?}").to_ascii_uppercase(), data))
            .collect::<Vec<(String, IconData)>>()
    });
    let icons = use_memo(props.query.clone(), |query| {
        let query = query.to_ascii_uppercase();
        let words = query.split(' ').collect::<Vec<_>>();
        initial_icons
            .iter()
            .map(|(title, data)| {
                let show = words.iter().all(|word| {
                    title
                        .contains(word)
                });
                (*data, show)
            })
            .collect::<Vec<(IconData, bool)>>()
    });

    if icons.is_empty() {
        return html! {
            <div class="no-icons">{"No Icons Found"}</div>
        };
    }

    html! {
        <div class="gallery">
            <>
                {icons.iter().copied().map(|(icon_data, show)| {
                    html_nested! {
                        <div style={if show { None } else { Some("display: none;") }}>
                            <GalleryItem {icon_data}/>
                        </div>
                    }
                }).collect::<Html>()}
            </>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct GalleryItemProps {
    icon_data: IconData,
}

#[function_component]
fn GalleryItem(props: &GalleryItemProps) -> Html {
    let icon_data = props.icon_data;
    let title = format!("{:?}", icon_data);
    let icon_name = title.clone();
    let timeout_ref = use_mut_ref(|| None);
    let show_copied = use_state(|| false);

    let onclick = {
        let title = title.clone();
        let show_copied = show_copied.clone();
        Callback::from(move |_: MouseEvent| {
            let window = window().unwrap();
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(&title);
            show_copied.set(true);
        })
    };

    use_effect_with(*show_copied, {
        let show_copied = show_copied.clone();
        move |show| {
            let window = window().unwrap();
            if *show {
                let closure = Closure::<dyn FnMut()>::new(move || {
                    show_copied.set(false);
                });

                let id = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        1000, // disappear after 1s
                    )
                    .unwrap();

                *timeout_ref.borrow_mut() = Some(id);
                closure.forget();
            }

            move || {
                if let Some(id) = timeout_ref.borrow_mut().take() {
                    window.clear_timeout_with_handle(id);
                }
            }
        }
    });

    html! {
        <div class="icon">
            <Icon
                {title}
                data={icon_data}
                width={"24"}
                height={"24"}
                onclick={onclick}
            />
            <p class="icon-name">{icon_name}</p>

            if *show_copied {
                <div class="copied-tooltip">
                    {"Copied"}
                </div>
            }
        </div>
    }
}
