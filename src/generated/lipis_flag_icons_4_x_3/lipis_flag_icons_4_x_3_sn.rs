use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_sn (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-sn" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" stroke - width = "1pt" > < path fill = "#0b7226" d = "M0 0h213.3v480H0z" /> < path fill = "#ff0" d = "M213.3 0h213.3v480H213.3z" /> < path fill = "#bc0000" d = "M426.6 0H640v480H426.6z" /> </ g > < path fill = "#0b7226" d = "M342 218.8h71.8l-56.6 43.6 20.7 69.3-56.6-43.6-56.6 41.6 20.7-67.3-56.6-43.6h69.8l22.7-71.3z" /> </ svg > } }