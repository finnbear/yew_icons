use crate :: IconProps ; # [inline (never)] pub fn lucide_timer_reset (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M10 2h4" /> < path d = "M12 14v-4" /> < path d = "M4 13a8 8 0 0 1 8-7 8 8 0 1 1-5.3 14L4 17.6" /> < path d = "M9 17H4v5" /> </ svg > } }