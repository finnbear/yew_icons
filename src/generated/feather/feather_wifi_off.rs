use crate :: IconProps ; # [inline (never)] pub fn feather_wifi_off (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < line x1 = "1" y1 = "1" x2 = "23" y2 = "23" /> < path d = "M16.72 11.06A10.94 10.94 0 0 1 19 12.55" /> < path d = "M5 12.55a10.94 10.94 0 0 1 5.17-2.39" /> < path d = "M10.71 5.05A16 16 0 0 1 22.58 9" /> < path d = "M1.42 9a15.91 15.91 0 0 1 4.7-2.88" /> < path d = "M8.53 16.11a6 6 0 0 1 6.95 0" /> < line x1 = "12" y1 = "20" x2 = "12.01" y2 = "20" /> </ svg > } }