use crate :: IconProps ; # [inline (never)] pub fn feather_repeat (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < polyline points = "17 1 21 5 17 9" /> < path d = "M3 11V9a4 4 0 0 1 4-4h14" /> < polyline points = "7 23 3 19 7 15" /> < path d = "M21 13v2a4 4 0 0 1-4 4H3" /> </ svg > } }