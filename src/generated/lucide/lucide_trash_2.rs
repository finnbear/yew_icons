use crate :: IconProps ; # [inline (never)] pub fn lucide_trash_2 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M3 6h18" /> < path d = "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /> < path d = "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" /> < line x1 = "10" y1 = "11" x2 = "10" y2 = "17" /> < line x1 = "14" y1 = "11" x2 = "14" y2 = "17" /> </ svg > } }