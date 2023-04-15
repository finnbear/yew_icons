use enum_iterator::IntoEnumIterator;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct GalleryProps {
    pub query: String,
}

#[function_component]
pub fn Gallery(props: &GalleryProps) -> Html {
    let initial_icons = use_memo(|_| IconId::into_enum_iter().collect::<Vec<IconId>>(), ());
    let icons = use_memo(
        |query| {
            initial_icons
                .iter()
                .filter(|icon_id| {
                    let title = format!("{:?}", icon_id);

                    query.to_ascii_lowercase().split(' ').all(|word| {
                        title
                            .to_ascii_lowercase()
                            .contains(&word.to_ascii_lowercase())
                    })
                })
                .cloned()
                .collect::<Vec<IconId>>()
        },
        props.query.clone(),
    );

    if icons.is_empty() {
        return html! {
            <div class="no-icons">{"No Icons Found"}</div>
        };
    }

    html! {
        <div class="gallery">
            <>
                {icons.iter().cloned().map(|icon_id| {
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
            }).collect::<Html>()}
            </>
        </div>
    }
}
