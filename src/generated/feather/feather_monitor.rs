use crate :: IconProps ; # [inline (never)] pub fn feather_monitor (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "2" y = "3" width = "20" height = "14" rx = "2" ry = "2" /> < line x1 = "8" y1 = "21" x2 = "16" y2 = "21" /> < line x1 = "12" y1 = "17" x2 = "12" y2 = "21" /> </ svg > } }