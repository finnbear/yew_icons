use crate :: IconProps ; # [inline (never)] pub fn lucide_hash (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < line x1 = "4" y1 = "9" x2 = "20" y2 = "9" /> < line x1 = "4" y1 = "15" x2 = "20" y2 = "15" /> < line x1 = "10" y1 = "3" x2 = "8" y2 = "21" /> < line x1 = "16" y1 = "3" x2 = "14" y2 = "21" /> </ svg > } }