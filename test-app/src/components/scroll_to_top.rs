use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, ScrollBehavior};
use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[derive(PartialEq, Properties)]
pub struct ScrollToTopProps {
    #[prop_or(200.0)]
    pub threshold: f64,
}

#[function_component]
pub fn ScrollToTop(props: &ScrollToTopProps) -> Html {
    let visible = use_state(|| can_show(props.threshold));

    let scroll_to_top = Callback::from(|_| {
        let window = window().unwrap();
        let mut options = web_sys::ScrollToOptions::new();
        options.top(0.0).behavior(ScrollBehavior::Smooth);

        window.scroll_with_scroll_to_options(&options);
    });

    {
        let visible = visible.clone();
        let threshold = props.threshold;

        use_effect(move || {
            let closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
                let show = can_show(threshold);
                if show {
                    if !*visible {
                        visible.set(true);
                    }
                } else {
                    if *visible {
                        visible.set(false);
                    }
                }
            });

            let window = window().unwrap();
            window
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .unwrap();

            move || {
                window
                    .remove_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();
            }
        });
    }

    if *visible {
        return Default::default();
    }

    html! {
        <button class="scroll-to-top-btn" onclick={scroll_to_top}>
            <Icon data={IconData::BOOTSTRAP_ARROW_UP_SHORT} />
        </button>
    }
}

fn can_show(threshold: f64) -> bool {
    let window = window().unwrap();
    threshold > window.scroll_y().unwrap_or_default()
}
