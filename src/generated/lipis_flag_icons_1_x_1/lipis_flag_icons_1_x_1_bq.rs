use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_bq (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-bq" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#21468b" d = "M0 0h512v512H0z" /> < path fill = "#fff" d = "M0 0h512v341.3H0z" /> < path fill = "#ae1c28" d = "M0 0h512v170.7H0z" /> </ svg > } }