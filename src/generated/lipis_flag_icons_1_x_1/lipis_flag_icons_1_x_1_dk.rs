use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_dk (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-dk" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#c8102e" d = "M0 0h512.1v512H0z" /> < path fill = "#fff" d = "M144 0h73.1v512H144z" /> < path fill = "#fff" d = "M0 219.4h512.1v73.2H0z" /> </ svg > } }