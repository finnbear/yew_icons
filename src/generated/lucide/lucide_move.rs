use crate :: IconProps ; # [inline (never)] pub fn lucide_move (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < polyline points = "5 9 2 12 5 15" /> < polyline points = "9 5 12 2 15 5" /> < polyline points = "15 19 12 22 9 19" /> < polyline points = "19 9 22 12 19 15" /> < line x1 = "2" y1 = "12" x2 = "22" y2 = "12" /> < line x1 = "12" y1 = "2" x2 = "12" y2 = "22" /> </ svg > } }