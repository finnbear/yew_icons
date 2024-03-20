// Generated file, do not edit by hand, see `src/generator.rs`

impl IconData {
    const EXTRA_GOOGLE_PLAY: Self = {
        #[inline(never)]
        fn extra_google_play(
            crate::IconProps {
                data: _,
                title,
                width,
                height,
                onclick,
                oncontextmenu,
                class,
                style,
                role,
            }: &crate::IconProps,
        ) -> yew::Html {
            yew::html! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 28.99 31.99" > if let Some (title) = title . clone () { < title > { title } </ title > } < g data - name = "Capa 2" >< g data - name = "Capa 1" >< path d = "M13.54 15.28.12 29.34a3.66 3.66 0 0 0 5.33 2.16l15.1-8.6Z" style = "fill:#ea4335" />< path d = "m27.11 12.89-6.53-3.74-7.35 6.45 7.38 7.28 6.48-3.7a3.54 3.54 0 0 0 1.5-4.79 3.62 3.62 0 0 0-1.5-1.5z" style = "fill:#fbbc04" />< path d = "M.12 2.66a3.57 3.57 0 0 0-.12.92v24.84a3.57 3.57 0 0 0 .12.92L14 15.64Z" style = "fill:#4285f4" />< path d = "m13.64 16 6.94-6.85L5.5.51A3.73 3.73 0 0 0 3.63 0 3.64 3.64 0 0 0 .12 2.65Z" style = "fill:#34a853" /></ g ></ g ></ svg > }
        }
        Self {
            name: "EXTRA_GOOGLE_PLAY",
            html: extra_google_play,
        }
    };
}
