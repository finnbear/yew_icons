use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_pm (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-pm" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" stroke - width = "1pt" > < path fill = "#fff" d = "M0 0h512v512H0z" /> < path fill = "#002654" d = "M0 0h170.7v512H0z" /> < path fill = "#ce1126" d = "M341.3 0H512v512H341.3z" /> </ g > </ svg > } }