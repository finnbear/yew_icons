use crate :: IconProps ; # [inline (never)] pub fn lucide_minimize_2 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < polyline points = "4 14 10 14 10 20" /> < polyline points = "20 10 14 10 14 4" /> < line x1 = "14" y1 = "10" x2 = "21" y2 = "3" /> < line x1 = "3" y1 = "21" x2 = "10" y2 = "14" /> </ svg > } }