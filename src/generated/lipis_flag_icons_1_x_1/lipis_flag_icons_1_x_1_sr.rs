use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_sr (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-sr" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#377e3f" d = "M0 0h512v512H0z" /> < path fill = "#fff" d = "M0 102.4h512v307.2H0z" /> < path fill = "#b40a2d" d = "M0 153.6h512v204.8H0z" /> < path fill = "#ecc81d" d = "m255.9 163.4 60.2 185.2-157.6-114.5h194.8L195.7 348.6z" /> </ svg > } }