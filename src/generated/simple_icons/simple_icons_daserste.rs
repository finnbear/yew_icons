use crate :: IconProps ; # [inline (never)] pub fn simple_icons_daserste (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11.646.005C5.158.2-.001 5.57 0 12.127.135 18.724 5.468 24 12 24s11.865-5.276 12-11.873C24.001 5.291 18.41-.195 11.645.005zm5.138 4.93V16.96L8.78 19.92v-9.08l-3.9 1.386V9.263l11.903-4.328z" /></ svg > } }