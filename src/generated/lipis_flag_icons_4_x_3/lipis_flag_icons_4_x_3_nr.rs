use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_nr (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-nr" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "nr-a" > < path fill - opacity = ".7" d = "M-54.7 0H628v512H-54.7z" /> </ clippath > </ defs > < g fill - rule = "evenodd" stroke - width = "1pt" transform = "translate(51.3) scale(.9375)" > < path fill = "#002170" d = "M-140 0H884v512H-140z" /> < path fill = "#ffb20d" d = "M-140 234.1H884V278H-140z" /> < path fill = "#fff" d = "m161.8 438-33-33-10.5 45.4-12-45-31.9 34 12.1-45L42 407.9l33-33-45.4-10.6 45-12-34-31.8 45 12L72 288l33 33 10.6-45.4 12 45 31.8-34-12 45 44.5-13.5-33 33 45.4 10.5-45 12 34 32-45-12.2z" /> </ g > </ svg > } }