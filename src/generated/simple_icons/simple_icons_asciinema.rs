use crate :: IconProps ; # [inline (never)] pub fn simple_icons_asciinema (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M1.61 0V24L22.39 12L1.61 0M5.76 7.2L10.06 9.68L5.76 12.16V7.2M12.55 11.12L14.08 12L5.76 16.8V15.04L12.55 11.12Z" /></ svg > } }