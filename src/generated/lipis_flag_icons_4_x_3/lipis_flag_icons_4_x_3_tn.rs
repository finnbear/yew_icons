use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_tn (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-tn" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "tn-a" > < path fill - opacity = ".7" d = "M-85.3 0h682.6v512H-85.3z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "translate(80) scale(.9375)" > < path fill = "#e70013" d = "M-128 0h768v512h-768z" /> < path fill = "#fff" d = "M385.8 255.8a129.1 129.1 0 1 1-258.2 0 129.1 129.1 0 0 1 258.2 0z" /> < path fill = "#e70013" d = "M256.7 341.4a85.7 85.7 0 0 1 0-171.3c11.8 0 25.3 2.8 34.4 9.5-62.6 2.3-78.5 55.5-78.5 76.9s10.1 69.1 78.5 76.2c-7.8 5-22.6 8.7-34.4 8.7z" /> < path fill = "#e70013" d = "m332.1 291.8-38.9-14.2-25.7 32.4 1.5-41.3-38.8-14.5 39.8-11.4 1.7-41.3 23.2 34.3 39.8-11-25.5 32.5z" /> </ g > </ svg > } }