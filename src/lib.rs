#![doc = include_str!("../README.md")]

#[cfg(not(feature = "_generator"))]
mod generated;

#[cfg(not(feature = "_generator"))]
use yew::prelude::*;

#[cfg(not(feature = "_generator"))]
use yew::virtual_dom::AttrValue;

#[cfg(not(feature = "_generator"))]
pub(crate) struct IconCollection {
    name: &'static str,
    license: &'static str,
}

/// SVG icon design. Enable icon collections, consisting of
/// associated constants, with feature flags.
#[derive(Copy, Clone)]
#[cfg(not(feature = "_generator"))]
pub struct IconData {
    collection: &'static IconCollection,
    name: &'static str,
    view_box: Option<&'static str>,
    fill: Option<&'static str>,
    stroke: Option<&'static str>,
    stroke_width: Option<&'static str>,
    stroke_linecap: Option<&'static str>,
    stroke_linejoin: Option<&'static str>,
    src: &'static str,
}

#[cfg(not(feature = "_generator"))]
impl PartialEq for IconData {
    fn eq(&self, other: &Self) -> bool {
        self.collection.name.eq(other.collection.name) && self.name.eq(other.name)
    }
}
#[cfg(not(feature = "_generator"))]
impl Eq for IconData {}
#[cfg(not(feature = "_generator"))]
impl Ord for IconData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.collection.name.cmp(other.collection.name).then_with(|| self.name.cmp(other.name))
    }
}
#[cfg(not(feature = "_generator"))]
impl PartialOrd for IconData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
#[cfg(not(feature = "_generator"))]
impl std::fmt::Debug for IconData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_str(self.collection.name)?;
        f.write_char('_')?;
        f.write_str(self.name)
    }
}

/// For customizing icon rendering. Only `data` is required.
#[cfg(not(feature = "_generator"))]
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
    /// For accessibility.
    #[prop_or(None)]
    pub role: Option<AttrValue>,
}

/// Renders a SVG icon. See [IconProps] for more information.
///
/// # Example
///
/// ```rust
/// use yew::prelude::*;
/// use yew_icons::{Icon, IconData};
///
/// let _ = html!{
///     <>
///         <Icon data={IconData::LUCIDE_ARROW_LEFT_CIRCLE}/>
///         <Icon data={IconData::LUCIDE_ARROW_UP_CIRCLE} width={"2em".to_owned()} height={"2em".to_owned()}/>
///         <Icon data={IconData::LUCIDE_ARROW_RIGHT_CIRCLE} onclick={Callback::from(|_: MouseEvent| {})}/>
///     </>
/// };
/// ```
#[cfg(not(feature = "_generator"))]
#[function_component(Icon)]
pub fn icon(IconProps {
    data,
    title,
    width,
    height,
    onclick,
    oncontextmenu,
    class,
    style,
    role,
}: &IconProps) -> Html {
    use yew::virtual_dom::AttrValue;
    yew::html!{
        <svg
            xmlns="http://www.w3.org/2000/svg"
            data-license={AttrValue::Static(data.collection.license)}
            viewBox={data.view_box.map(AttrValue::Static)}
            fill={data.fill.map(AttrValue::Static)}
            stroke={data.stroke.map(AttrValue::Static)}
            stroke-width={data.stroke_width.map(AttrValue::Static)}
            stroke-linecap={data.stroke_linecap.map(AttrValue::Static)}
            stroke-linejoin={data.stroke_linejoin.map(AttrValue::Static)}
            width={width.clone()}
            height={height.clone()}
            onclick={onclick.clone()}
            oncontextmenu={oncontextmenu.clone()}
            class={class.clone()}
            style={style.clone()}
            role={role.clone()}
            aria-hidden={if role.is_none() { Some("true") } else { None }}
        >
            if let Some(title) = title.clone() {
                <title>{title}</title>
            }
            {Html::from_html_unchecked(AttrValue::Static(data.src))}
        </svg>
    }
}

#[cfg(test)]
mod test {
    use crate::{Icon, IconData, IconProps};
    use yew::prelude::*;

    #[tokio::test]
    async fn test() {
        let data = IconData::LUCIDE_BEAKER;
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
