use crate :: IconProps ; # [inline (never)] pub fn lucide_lamp_wall_down (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11 13h6l3 7H8l3-7Z" /> < path d = "M14 13V8a2 2 0 0 0-2-2H8" /> < path d = "M4 9h2a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H4v6Z" /> </ svg > } }