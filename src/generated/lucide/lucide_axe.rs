use crate :: IconProps ; # [inline (never)] pub fn lucide_axe (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "m14 12-8.501 8.501a2.12 2.12 0 0 1-2.998 0h-.002a2.12 2.12 0 0 1 0-2.998L11 9.002" /> < path d = "m9 7 4-4 6 6h3l-.13.648a7.648 7.648 0 0 1-5.081 5.756L15 16v-3z" /> </ svg > } }