use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_tl (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-tl" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "tl-a" > < path fill - opacity = ".7" d = "M0 0h496v496H0z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "scale(1.0321)" > < path fill = "#cb000f" d = "M0 0h999v496H0z" /> < path fill = "#f8c00c" d = "M0 0c3.1 0 496 248.7 496 248.7L0 496.1V0z" /> < path d = "M0 0c2 0 330 248.7 330 248.7L0 496.1V0z" /> < path fill = "#fff" d = "m181.9 288.9-59-13L93 327l-5-57.8-58.8-13 53.1-24-3.2-57.5 39 42 53.6-24.4-28 52.2 38 44.4z" /> </ g > </ svg > } }