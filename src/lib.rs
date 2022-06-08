#[cfg(not(feature = "generator"))]
mod generated;
#[cfg(not(feature = "generator"))]
pub use generated::{get_svg, IconId};

#[cfg(not(feature = "generator"))]
use yew::prelude::*;

#[cfg(not(feature = "generator"))]
use yew::virtual_dom::AttrValue;

#[cfg(not(feature = "generator"))]
#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub icon_id: IconId,
    #[prop_or("24".into())]
    pub width: AttrValue,
    #[prop_or("24".into())]
    pub height: AttrValue,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[cfg(not(feature = "generator"))]
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    get_svg(
        props.icon_id,
        props.width.clone(),
        props.height.clone(),
        props.onclick.clone(),
    )
}

#[cfg(test)]
mod test {
    use crate::{Icon, IconId, IconProps};
    use enum_iterator::IntoEnumIterator;
    use yew::prelude::*;

    #[tokio::test]
    async fn test() {
        for icon_id in IconId::into_enum_iter() {
            let renderer = yew::ServerRenderer::<Icon>::with_props(IconProps {
                icon_id,
                width: "2em".into(),
                height: "3em".into(),
                onclick: Some(Callback::from(|_e: MouseEvent| {})),
            });

            let rendered = renderer.render().await;

            assert!(rendered.contains("2em"), "{:?} {}", icon_id, rendered);
            assert!(rendered.contains("3em"), "{:?} {}", icon_id, rendered);

            //println!("{:?} => {}", icon_id, rendered);
        }
    }
}
