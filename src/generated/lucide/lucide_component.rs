use crate :: IconProps ; # [inline (never)] pub fn lucide_component (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M5.5 8.5 9 12l-3.5 3.5L2 12l3.5-3.5Z" /> < path d = "m12 2 3.5 3.5L12 9 8.5 5.5 12 2Z" /> < path d = "M18.5 8.5 22 12l-3.5 3.5L15 12l3.5-3.5Z" /> < path d = "m12 15 3.5 3.5L12 22l-3.5-3.5L12 15Z" /> </ svg > } }