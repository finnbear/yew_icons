use crate :: IconProps ; # [inline (never)] pub fn lucide_shield_close (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" /> < line x1 = "9.5" y1 = "9" x2 = "14.5" y2 = "14" /> < line x1 = "14.5" y1 = "9" x2 = "9.5" y2 = "14" /> </ svg > } }