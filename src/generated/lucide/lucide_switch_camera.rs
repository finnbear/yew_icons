use crate :: IconProps ; # [inline (never)] pub fn lucide_switch_camera (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5" /> < path d = "M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5" /> < circle cx = "12" cy = "12" r = "3" /> < path d = "m18 22-3-3 3-3" /> < path d = "m6 2 3 3-3 3" /> </ svg > } }