use crate :: IconProps ; # [inline (never)] pub fn lucide_italic (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < line x1 = "19" y1 = "4" x2 = "10" y2 = "4" /> < line x1 = "14" y1 = "20" x2 = "5" y2 = "20" /> < line x1 = "15" y1 = "4" x2 = "9" y2 = "20" /> </ svg > } }