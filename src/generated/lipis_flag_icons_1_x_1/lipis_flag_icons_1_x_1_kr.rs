use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_kr (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-kr" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs id = "defs87" > < clippath id = "kr-a" > < path id = "path84" fill - opacity = ".7" d = "M-95.8-.4h682.7v512H-95.8Z" /> </ clippath > </ defs > < path id = "path526" fill = "#fff" d = "M0 0h512v512H0Z" style = "fill-rule:evenodd;stroke-width:8.7099" /> < g id = "g540" style = "fill-rule:evenodd" transform = "rotate(-56.3 367.2 -111.2) scale(9.375)" > < g id = "b2" > < path id = "b" d = "M-6-26H6v2H-6Zm0 3H6v2H-6Zm0 3H6v2H-6Z" /> < use href = "#b" id = "use529" width = "100%" height = "100%" x = "0" y = "44" /> </ g > < path id = "path532" stroke = "#fff" d = "M0 17v10" /> < path id = "path534" fill = "#cd2e3a" d = "M0-12a12 12 0 0 1 0 24Z" /> < path id = "path536" fill = "#0047a0" d = "M0-12a12 12 0 0 0 0 24A6 6 0 0 0 0 0Z" /> < circle id = "circle538" cx = "0" cy = "-6" r = "6" fill = "#cd2e3a" /> </ g > < g id = "g546" style = "fill-rule:evenodd" transform = "rotate(-123.7 196.5 59.5) scale(9.375)" > < use href = "#b2" id = "use542" width = "100%" height = "100%" x = "0" y = "0" /> < path id = "path544" stroke = "#fff" d = "M0-23.5v3M0 17v3.5m0 3v3" /> </ g > </ svg > } }