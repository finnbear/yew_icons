use crate :: IconProps ; # [inline (never)] pub fn lucide_alarm_clock (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < circle cx = "12" cy = "13" r = "8" /> < path d = "M12 9v4l2 2" /> < path d = "M5 3 2 6" /> < path d = "m22 6-3-3" /> < path d = "m6 19-2 2" /> < path d = "m18 19 2 2" /> </ svg > } }