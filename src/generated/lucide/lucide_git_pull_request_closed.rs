use crate :: IconProps ; # [inline (never)] pub fn lucide_git_pull_request_closed (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < circle cx = "18" cy = "18" r = "3" /> < circle cx = "6" cy = "6" r = "3" /> < path d = "M18 11.5V15" /> < path d = "m21 3-6 6" /> < path d = "m21 9-6-6" /> < line x1 = "6" y1 = "9" x2 = "6" y2 = "21" /> </ svg > } }