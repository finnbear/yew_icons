use crate :: IconProps ; # [inline (never)] pub fn lucide_pen_tool (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "m12 19 7-7 3 3-7 7-3-3z" /> < path d = "m18 13-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" /> < path d = "m2 2 7.586 7.586" /> < circle cx = "11" cy = "11" r = "2" /> </ svg > } }