use crate :: IconProps ; # [inline (never)] pub fn lucide_speaker (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "4" y = "2" width = "16" height = "20" rx = "2" ry = "2" /> < circle cx = "12" cy = "14" r = "4" /> < line x1 = "12" y1 = "6" x2 = "12.01" y2 = "6" /> </ svg > } }