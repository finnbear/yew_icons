use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_sl (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-sl" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "sl-a" > < rect width = "384" height = "512" rx = "4.6" ry = "7.6" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "scale(1.33333 1)" > < path fill = "#0000cd" d = "M0 341.7h512V512H0z" /> < path fill = "#fff" d = "M0 171.4h512v170.3H0z" /> < path fill = "#00cd00" d = "M0 0h512v171.4H0z" /> </ g > </ svg > } }