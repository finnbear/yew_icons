use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_am (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-am" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#d90012" d = "M0 0h640v160H0z" /> < path fill = "#0033a0" d = "M0 160h640v160H0z" /> < path fill = "#f2a800" d = "M0 320h640v160H0z" /> </ svg > } }