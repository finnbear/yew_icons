use crate :: IconProps ; # [inline (never)] pub fn feather_align_center (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < line x1 = "18" y1 = "10" x2 = "6" y2 = "10" /> < line x1 = "21" y1 = "6" x2 = "3" y2 = "6" /> < line x1 = "21" y1 = "14" x2 = "3" y2 = "14" /> < line x1 = "18" y1 = "18" x2 = "6" y2 = "18" /> </ svg > } }