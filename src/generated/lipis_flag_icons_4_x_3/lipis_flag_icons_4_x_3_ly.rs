use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_ly (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-ly" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "ly-a" > < path d = "M166.7-20h666.6v500H166.7z" /> </ clippath > </ defs > < g transform = "matrix(.96 0 0 .96 -160 19.2)" > < path fill = "#239e46" d = "M0-20h1000v500H0z" /> < path d = "M0-20h1000v375H0z" /> < path fill = "#e70013" d = "M0-20h1000v125H0z" /> < path fill = "#fff" d = "M544.2 185.8a54.3 54.3 0 1 0 0 88.4 62.5 62.5 0 1 1 0-88.4M530.4 230l84.1-27.3-52 71.5v-88.4l52 71.5z" /> </ g > </ svg > } }