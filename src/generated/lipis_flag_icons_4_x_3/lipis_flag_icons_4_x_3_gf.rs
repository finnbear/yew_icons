use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_gf (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-gf" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" stroke - width = "1pt" > < path fill = "#fff" d = "M0 0h640v480H0z" /> < path fill = "#002654" d = "M0 0h213.3v480H0z" /> < path fill = "#ce1126" d = "M426.7 0H640v480H426.7z" /> </ g > </ svg > } }