use crate :: IconProps ; # [inline (never)] pub fn lucide_maximize (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M8 3H5a2 2 0 0 0-2 2v3" /> < path d = "M21 8V5a2 2 0 0 0-2-2h-3" /> < path d = "M3 16v3a2 2 0 0 0 2 2h3" /> < path d = "M16 21h3a2 2 0 0 0 2-2v-3" /> </ svg > } }