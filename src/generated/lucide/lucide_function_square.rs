use crate :: IconProps ; # [inline (never)] pub fn lucide_function_square (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "3" y = "3" width = "18" height = "18" rx = "2" ry = "2" /> < path d = "M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3" /> < path d = "M9 11.2h5.7" /> </ svg > } }