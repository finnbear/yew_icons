use crate :: IconProps ; # [inline (never)] pub fn lucide_calculator (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "4" y = "2" width = "16" height = "20" rx = "2" /> < line x1 = "8" x2 = "16" y1 = "6" y2 = "6" /> < line x1 = "16" x2 = "16" y1 = "14" y2 = "18" /> < path d = "M16 10h.01" /> < path d = "M12 10h.01" /> < path d = "M8 10h.01" /> < path d = "M12 14h.01" /> < path d = "M8 14h.01" /> < path d = "M12 18h.01" /> < path d = "M8 18h.01" /> </ svg > } }