use crate :: IconProps ; # [inline (never)] pub fn lucide_scissors (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < circle cx = "6" cy = "6" r = "3" /> < circle cx = "6" cy = "18" r = "3" /> < line x1 = "20" y1 = "4" x2 = "8.12" y2 = "15.88" /> < line x1 = "14.47" y1 = "14.48" x2 = "20" y2 = "20" /> < line x1 = "8.12" y1 = "8.12" x2 = "12" y2 = "12" /> </ svg > } }