use crate :: IconProps ; # [inline (never)] pub fn simple_icons_roblox (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "m13.383 14.341-3.726-.958.959-3.726 3.726.959-.96 3.726zM4.913 0 0 19.088 19.088 24 24 4.912 4.912 0z" /></ svg > } }