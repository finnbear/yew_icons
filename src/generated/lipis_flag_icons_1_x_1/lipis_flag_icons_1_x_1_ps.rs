use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_ps (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-ps" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "ps-a" > < path fill - opacity = ".7" d = "M237.1 0h493.5v493.5H237.1z" /> </ clippath > </ defs > < g transform = "translate(-246) scale(1.0375)" > < g fill - rule = "evenodd" stroke - width = "1pt" > < path d = "M0 0h987v164.5H0z" /> < path fill = "#fff" d = "M0 164.5h987V329H0z" /> < path fill = "#090" d = "M0 329h987v164.5H0z" /> < path fill = "red" d = "m0 493.5 493.5-246.8L0 0v493.5z" /> </ g > </ g > </ svg > } }