use crate :: IconProps ; # [inline (never)] pub fn feather_truck (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect x = "1" y = "3" width = "15" height = "13" /> < polygon points = "16 8 20 8 23 11 23 16 16 16 16 8" /> < circle cx = "5.5" cy = "18.5" r = "2.5" /> < circle cx = "18.5" cy = "18.5" r = "2.5" /> </ svg > } }