use crate :: IconProps ; # [inline (never)] pub fn lucide_layout_list (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "3" y = "14" width = "7" height = "7" /> < rect x = "3" y = "3" width = "7" height = "7" /> < line x1 = "14" y1 = "4" x2 = "21" y2 = "4" /> < line x1 = "14" y1 = "9" x2 = "21" y2 = "9" /> < line x1 = "14" y1 = "15" x2 = "21" y2 = "15" /> < line x1 = "14" y1 = "20" x2 = "21" y2 = "20" /> </ svg > } }