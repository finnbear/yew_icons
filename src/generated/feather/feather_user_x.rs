use crate :: IconProps ; # [inline (never)] pub fn feather_user_x (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /> < circle cx = "8.5" cy = "7" r = "4" /> < line x1 = "18" y1 = "8" x2 = "23" y2 = "13" /> < line x1 = "23" y1 = "8" x2 = "18" y2 = "13" /> </ svg > } }