use crate :: IconProps ; # [inline (never)] pub fn lucide_landmark (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < line x1 = "3" y1 = "22" x2 = "21" y2 = "22" /> < line x1 = "6" y1 = "18" x2 = "6" y2 = "11" /> < line x1 = "10" y1 = "18" x2 = "10" y2 = "11" /> < line x1 = "14" y1 = "18" x2 = "14" y2 = "11" /> < line x1 = "18" y1 = "18" x2 = "18" y2 = "11" /> < polygon points = "12 2 20 7 4 7" /> </ svg > } }