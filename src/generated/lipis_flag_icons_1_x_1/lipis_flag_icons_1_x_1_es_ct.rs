use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_es_ct (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-es-ct" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#fcdd09" d = "M0 0h512v512H0z" /> < path stroke = "#da121a" stroke - width = "60" d = "M0 90h810m0 120H0m0 120h810m0 120H0" transform = "scale(.6321 .94815)" /> </ svg > } }