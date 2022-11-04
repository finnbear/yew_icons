#![doc = include_str!("../README.md")]

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
    /// Which icon to render. Enable icons with feature flags.
    pub icon_id: IconId,
    /// Tooltip text.
    pub title: Option<AttrValue>,
    /// CSS width.
    #[prop_or("24".into())]
    pub width: AttrValue,
    /// CSS height.
    #[prop_or("24".into())]
    pub height: AttrValue,
    /// Callback when icon is clicked.
    pub onclick: Option<Callback<MouseEvent>>,
    /// Callback when icon is subject to context menu (usually means it was right-clicked).
    pub oncontextmenu: Option<Callback<MouseEvent>>,
    /// For CSS.
    #[prop_or_default]
    pub class: Classes,
    /// For inline CSS.
    pub style: Option<AttrValue>,
}

/// Renders a SVG icon. See [IconProps] for more information.
///
/// # Example
///
/// ```rust
///use yew::prelude::*;
// use yew_icons::{Icon, IconId};
//
// html!{
//     <>
//         <Icon icon_id={IconId::LucideArrowLeftCircle}/>
//         <Icon icon_id={IconId::LucideArrowUpCircle} width={"2em".to_owned()} height={"2em".to_owned()}/>
//         <Icon icon_id={IconId::LucideArrowRightCircle} onclick={Callback::from(|_: MouseEvent| {})}/>
//     </>
// }
/// ```
#[cfg(not(feature = "generator"))]
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    get_svg(props)
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
