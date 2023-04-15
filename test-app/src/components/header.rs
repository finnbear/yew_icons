pub use yew::prelude::*;
use yew_icons::{Icon, IconId};
use crate::components::ThemeToggle;

#[function_component]
pub fn Header() -> Html {
    html! {
        <header>
            <h1 style={"font-family: sans-serif; margin-top: 0;"}>{"yew_icons"}</h1>
            <p class="header-links">
                <a title={"crates.io"} href={"https://crates.io/crates/yew_icons"} target="_blank" rel="noopener">
                    {"crates.io"}
                </a>
                {" • "}
                <a title={"GitHub"} href={"https://github.com/finnbear/yew_icons"} target="_blank" rel="noopener">
                    <Icon
                        class="github"
                        title={"GitHub"}
                        icon_id={IconId::BootstrapGithub}
                        height={"30"}
                        width={"30"}
                    />
                </a>
                <ThemeToggle/>
            </p>
        </header>
    }
}
