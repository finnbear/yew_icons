use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_cd (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-cd" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "cd-a" > < path fill = "#fff" d = "M0-88h600v600H0z" /> </ clippath > </ defs > < g transform = "matrix(.853 0 0 .853 0 75.1)" > < path fill = "#007fff" d = "M0-88h800v600H0z" /> < path fill = "#f7d618" d = "M36 32h84l26-84 26 84h84l-68 52 26 84-68-52-68 52 26-84-68-52zM750-88 0 362v150h50L800 62V-88h-50" /> < path fill = "#ce1021" d = "M800-88 0 392v120L800 32V-88" /> </ g > </ svg > } }