use crate :: IconProps ; # [inline (never)] pub fn lucide_codepen (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < polygon points = "12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2" /> < line x1 = "12" y1 = "22" x2 = "12" y2 = "15.5" /> < polyline points = "22 8.5 12 15.5 2 8.5" /> < polyline points = "2 15.5 12 8.5 22 15.5" /> < line x1 = "12" y1 = "2" x2 = "12" y2 = "8.5" /> </ svg > } }