use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_mu (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-mu" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" > < path fill = "#009f4d" d = "M0 384h512v128H0z" /> < path fill = "#151f6d" d = "M0 128h512v128H0z" /> < path fill = "#ee2737" d = "M0 0h512v128H0z" /> < path fill = "#ffcd00" d = "M0 256h512v128H0z" /> </ g > </ svg > } }