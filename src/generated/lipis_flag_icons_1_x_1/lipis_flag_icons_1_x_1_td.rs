use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_td (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-td" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" > < path fill = "#000067" d = "M0 0h171.2v512H0z" /> < path fill = "red" d = "M340.8 0H512v512H340.8z" /> < path fill = "#ff0" d = "M171.2 0h169.6v512H171.2z" /> </ g > </ svg > } }