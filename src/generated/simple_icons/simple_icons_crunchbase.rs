use crate :: IconProps ; # [inline (never)] pub fn simple_icons_crunchbase (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M21.6 0H2.4A2.41 2.41 0 0 0 0 2.4v19.2A2.41 2.41 0 0 0 2.4 24h19.2a2.41 2.41 0 0 0 2.4-2.4V2.4A2.41 2.41 0 0 0 21.6 0zM7.045 14.465A2.11 2.11 0 0 0 9.84 13.42h1.66a3.69 3.69 0 1 1 0-1.75H9.84a2.11 2.11 0 1 0-2.795 2.795zm11.345.845a3.55 3.55 0 0 1-1.06.63 3.68 3.68 0 0 1-3.39-.38v.38h-1.51V5.37h1.5v4.11a3.74 3.74 0 0 1 1.8-.63H16a3.67 3.67 0 0 1 2.39 6.46zm-.223-2.766a2.104 2.104 0 1 1-4.207 0 2.104 2.104 0 0 1 4.207 0z" /></ svg > } }