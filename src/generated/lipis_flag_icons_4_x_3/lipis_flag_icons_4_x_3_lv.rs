use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_lv (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-lv" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" > < path fill = "#fff" d = "M0 0h640v480H0z" /> < path fill = "#981e32" d = "M0 0h640v192H0zm0 288h640v192H0z" /> </ g > </ svg > } }