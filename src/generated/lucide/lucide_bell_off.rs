use crate :: IconProps ; # [inline (never)] pub fn lucide_bell_off (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M13.73 21a2 2 0 0 1-3.46 0" /> < path d = "M18.63 13A17.888 17.888 0 0 1 18 8" /> < path d = "M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14" /> < path d = "M18 8a6 6 0 0 0-9.33-5" /> < path d = "m2 2 20 20" /> </ svg > } }