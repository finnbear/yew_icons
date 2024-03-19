use crate::components::ThemeToggle;
pub use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component]
pub fn Header() -> Html {
    html! {
        <header>
            <h1 style={"font-family: sans-serif; margin-top: 0;"}>{"yew_icons"}</h1>
            <p class="header-links">
                <a title={"crates.io"} href={"https://crates.io/crates/yew_icons"} target="_blank" rel="noopener">
                    {"crates.io"}
                </a>
                <a title={"docs"} href={"https://docs.rs/yew_icons/latest/yew_icons"} target="_blank" rel="noopener">
                    {"docs"}
                </a>
                {" • "}
                <a title={"GitHub"} href={"https://github.com/finnbear/yew_icons"} target="_blank" rel="noopener">
                    <Icon
                        class="github"
                        title={"GitHub"}
                        data={IconData::BOOTSTRAP_GITHUB}
                        height={"30"}
                        width={"30"}
                    />
                </a>
                <ThemeToggle/>
            </p>
        </header>
    }
}
