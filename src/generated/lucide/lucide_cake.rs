use crate :: IconProps ; # [inline (never)] pub fn lucide_cake (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M20 21v-8a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8" /> < path d = "M4 16s.5-1 2-1 2.5 2 4 2 2.5-2 4-2 2.5 2 4 2 2-1 2-1" /> < path d = "M2 21h20" /> < path d = "M7 8v2" /> < path d = "M12 8v2" /> < path d = "M17 8v2" /> < path d = "M7 4h.01" /> < path d = "M12 4h.01" /> < path d = "M17 4h.01" /> </ svg > } }