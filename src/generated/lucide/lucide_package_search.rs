use crate :: IconProps ; # [inline (never)] pub fn lucide_package_search (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" /> < path d = "M16.5 9.4 7.55 4.24" /> < polyline points = "3.29 7 12 12 20.71 7" /> < line x1 = "12" y1 = "22" x2 = "12" y2 = "12" /> < circle cx = "18.5" cy = "15.5" r = "2.5" /> < path d = "M20.27 17.27 22 19" /> </ svg > } }