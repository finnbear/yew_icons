use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_mr (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-mr" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#cd2a3e" d = "M0 0h640v480H0z" /> < path fill = "#006233" d = "M0 72h640v336H0z" /> < path fill = "#ffc400" d = "M470 154.6a150 150 0 0 1-300 0 154.9 154.9 0 0 0-5 39.2 155 155 0 1 0 310 0 154.4 154.4 0 0 0-5-39.2z" /> < path fill = "#ffc400" d = "m320 93.8-13.5 41.5H263l35.3 25.6-13.5 41.4 35.3-25.6 35.3 25.6-13.5-41.4 35.3-25.6h-43.6z" /> </ svg > } }