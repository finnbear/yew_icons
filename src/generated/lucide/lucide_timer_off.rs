use crate :: IconProps ; # [inline (never)] pub fn lucide_timer_off (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M10 2h4" /> < path d = "M4.6 11a8 8 0 0 0 1.7 8.7 8 8 0 0 0 8.7 1.7" /> < path d = "M7.4 7.4a8 8 0 0 1 10.3 1 8 8 0 0 1 .9 10.2" /> < path d = "m2 2 20 20" /> < path d = "M12 12v-2" /> </ svg > } }