use crate :: IconProps ; # [inline (never)] pub fn lucide_power_off (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M18.36 6.64A9 9 0 0 1 20.77 15" /> < path d = "M6.16 6.16a9 9 0 1 0 12.68 12.68" /> < path d = "M12 2v4" /> < path d = "m2 2 20 20" /> </ svg > } }