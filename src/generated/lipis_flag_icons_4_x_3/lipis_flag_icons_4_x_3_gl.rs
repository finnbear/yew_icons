use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_gl (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-gl" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#fff" d = "M0 0h640v480H0z" /> < path fill = "#d00c33" d = "M0 240h640v240H0zm80 0a160 160 0 1 0 320 0 160 160 0 0 0-320 0" /> </ svg > } }