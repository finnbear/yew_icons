use crate :: IconProps ; # [inline (never)] pub fn lucide_currency (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < circle cx = "12" cy = "12" r = "8" /> < line x1 = "3" y1 = "3" x2 = "6" y2 = "6" /> < line x1 = "21" y1 = "3" x2 = "18" y2 = "6" /> < line x1 = "3" y1 = "21" x2 = "6" y2 = "18" /> < line x1 = "21" y1 = "21" x2 = "18" y2 = "18" /> </ svg > } }