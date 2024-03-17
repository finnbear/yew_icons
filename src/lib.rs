#![doc = include_str!("../README.md")]

#[cfg(not(feature = "generator"))]
mod generated;

#[cfg(not(feature = "generator"))]
use yew::prelude::*;

#[cfg(not(feature = "generator"))]
use yew::virtual_dom::AttrValue;

#[derive(Copy, Clone)]
#[cfg(not(feature = "generator"))]
pub struct IconData {
    name: &'static str,
    html: fn(props: &IconProps) -> Html,
}

#[cfg(not(feature = "generator"))]
impl IconData {
    const HELLO_WORLD: IconData = IconData {
        name: "HelloWorld",
        html: |props: &IconProps| {
            html! {
                <p>{"Hello world"}</p>
            }
        },
    };
}

#[cfg(not(feature = "generator"))]
impl PartialEq for IconData {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}
#[cfg(not(feature = "generator"))]
impl Eq for IconData {}
#[cfg(not(feature = "generator"))]
impl Ord for IconData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(other.name)
    }
}
#[cfg(not(feature = "generator"))]
impl PartialOrd for IconData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(not(feature = "generator"))]
impl Debug for IconData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

/// For customizing icon rendering. Only `icon_id` is required.
#[cfg(not(feature = "generator"))]
#[derive(Properties, PartialEq)]
pub struct IconProps {
    /// Which icon to render. Enable icon collections with feature flags.
    pub data: IconData,
    /// Tooltip text.
    #[prop_or(None)]
    pub title: Option<AttrValue>,
    /// CSS width.
    #[prop_or("24".into())]
    pub width: AttrValue,
    /// CSS height.
    #[prop_or("24".into())]
    pub height: AttrValue,
    /// Callback when icon is clicked.
    #[prop_or(None)]
    pub onclick: Option<Callback<MouseEvent>>,
    /// Callback when icon is subject to context menu (usually means it was right-clicked).
    #[prop_or(None)]
    pub oncontextmenu: Option<Callback<MouseEvent>>,
    /// For CSS.
    #[prop_or_default]
    pub class: Classes,
    /// For inline CSS.
    #[prop_or(None)]
    pub style: Option<AttrValue>,
    #[prop_or(None)]
    pub role: Option<AttrValue>,
}

/// Renders a SVG icon. See [IconProps] for more information.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_icons::{Icon, IconId};
///
/// html!{
///     <>
///         <Icon icon_id={IconId::LucideArrowLeftCircle}/>
///         <Icon icon_id={IconId::LucideArrowUpCircle} width={"2em".to_owned()} height={"2em".to_owned()}/>
///         <Icon icon_id={IconId::LucideArrowRightCircle} onclick={Callback::from(|_: MouseEvent| {})}/>
///     </>
/// }
/// ```
#[cfg(not(feature = "generator"))]
#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    (props.data.html)(props)
}

#[cfg(test)]
mod test {
    use crate::{Icon, IconData, IconProps};
    use yew::prelude::*;

    #[tokio::test]
    async fn test() {
        let data = IconData::HELLO_WORLD;
        let renderer = yew::ServerRenderer::<Icon>::with_props(move || IconProps {
            data,
            width: "2em".into(),
            height: "3em".into(),
            onclick: Some(Callback::from(|_e: MouseEvent| {})),
            class: Classes::new(),
            oncontextmenu: None,
            style: None,
            title: None,
            role: Some("presentation".into()),
        });

        let rendered = renderer.render().await;

        assert!(rendered.contains("2em"), "{data:?} {}", rendered);
        assert!(rendered.contains("3em"), "{data:?} {}", rendered);

        //println!("{:?} => {}", icon_id, rendered);
    }
}
