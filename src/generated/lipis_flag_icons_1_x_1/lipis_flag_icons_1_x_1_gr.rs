use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_gr (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-gr" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#005bae" fill - rule = "evenodd" stroke - width = ".8" d = "M0 0h512v57H0z" /> < path fill = "#fff" fill - rule = "evenodd" stroke - width = ".8" d = "M0 57h512v57H0z" /> < path fill = "#005bae" fill - rule = "evenodd" stroke - width = ".8" d = "M0 114h512v57H0z" /> < path fill = "#fff" fill - rule = "evenodd" stroke - width = ".8" d = "M0 171h512v57H0z" /> < path fill = "#005bae" fill - rule = "evenodd" stroke - width = ".8" d = "M0 228h512v56.9H0z" /> < path fill = "#fff" fill - rule = "evenodd" stroke - width = ".8" d = "M0 284.9h512v57H0z" /> < path fill = "#005bae" fill - rule = "evenodd" stroke - width = ".8" d = "M0 341.9h512v57H0z" /> < path fill = "#fff" fill - rule = "evenodd" stroke - width = ".8" d = "M0 398.9h512v57H0z" /> < path fill = "#005bae" stroke - width = "3" d = "M0 0h284.9v284.9H0z" /> < g fill = "#fff" fill - rule = "evenodd" stroke - width = "1.3" > < path d = "M148 0h74v370h-74z" transform = "scale(.77)" /> < path d = "M0 148h370v74H0z" transform = "scale(.77)" /> </ g > < path fill = "#005bae" fill - rule = "evenodd" stroke - width = ".8" d = "M0 455h512v57H0z" /> </ svg > } }