use crate :: IconProps ; # [inline (never)] pub fn lucide_calendar_off (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M4.18 4.18A2 2 0 0 0 3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 1.82-1.18" /> < path d = "M21 15.5V6a2 2 0 0 0-2-2H9.5" /> < path d = "M16 2v4" /> < path d = "M3 10h7" /> < path d = "M21 10h-5.5" /> < line x1 = "2" y1 = "2" x2 = "22" y2 = "22" /> </ svg > } }