use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_ci (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-ci" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" > < path fill = "#00cd00" d = "M426.8 0H640v480H426.8z" /> < path fill = "#ff9a00" d = "M0 0h212.9v480H0z" /> < path fill = "#fff" d = "M212.9 0h214v480h-214z" /> </ g > </ svg > } }