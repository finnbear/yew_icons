use crate :: IconProps ; # [inline (never)] pub fn lucide_arrow_up_down (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < polyline points = "11 17 7 21 3 17" /> < line x1 = "7" y1 = "21" x2 = "7" y2 = "9" /> < polyline points = "21 7 17 3 13 7" /> < line x1 = "17" y1 = "15" x2 = "17" y2 = "3" /> </ svg > } }