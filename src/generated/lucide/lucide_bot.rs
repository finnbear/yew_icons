use crate :: IconProps ; # [inline (never)] pub fn lucide_bot (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "3" y = "11" width = "18" height = "10" rx = "2" /> < circle cx = "12" cy = "5" r = "2" /> < path d = "M12 7v4" /> < line x1 = "8" y1 = "16" x2 = "8" y2 = "16" /> < line x1 = "16" y1 = "16" x2 = "16" y2 = "16" /> </ svg > } }