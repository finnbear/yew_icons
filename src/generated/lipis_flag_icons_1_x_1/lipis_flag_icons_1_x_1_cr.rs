use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_cr (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-cr" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" stroke - width = "1pt" > < path fill = "#0000b4" d = "M0 0h512v512H0z" /> < path fill = "#fff" d = "M0 80.5h512v343.7H0z" /> < path fill = "#d90000" d = "M0 168.2h512v168.2H0z" /> </ g > </ svg > } }