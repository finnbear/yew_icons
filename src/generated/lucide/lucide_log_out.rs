use crate :: IconProps ; # [inline (never)] pub fn lucide_log_out (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" /> < polyline points = "16 17 21 12 16 7" /> < line x1 = "21" y1 = "12" x2 = "9" y2 = "12" /> </ svg > } }