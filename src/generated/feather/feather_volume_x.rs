use crate :: IconProps ; # [inline (never)] pub fn feather_volume_x (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < polygon points = "11 5 6 9 2 9 2 15 6 15 11 19 11 5" /> < line x1 = "23" y1 = "9" x2 = "17" y2 = "15" /> < line x1 = "17" y1 = "9" x2 = "23" y2 = "15" /> </ svg > } }