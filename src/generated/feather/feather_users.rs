use crate :: IconProps ; # [inline (never)] pub fn feather_users (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /> < circle cx = "9" cy = "7" r = "4" /> < path d = "M23 21v-2a4 4 0 0 0-3-3.87" /> < path d = "M16 3.13a4 4 0 0 1 0 7.75" /> </ svg > } }