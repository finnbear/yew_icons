use crate :: IconProps ; # [inline (never)] pub fn lucide_bed_double (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8" /> < path d = "M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4" /> < path d = "M12 4v6" /> < path d = "M2 18h20" /> </ svg > } }