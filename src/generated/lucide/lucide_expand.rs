use crate :: IconProps ; # [inline (never)] pub fn lucide_expand (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "m21 21-6-6m6 6v-4.8m0 4.8h-4.8" /> < path d = "M3 16.2V21m0 0h4.8M3 21l6-6" /> < path d = "M21 7.8V3m0 0h-4.8M21 3l-6 6" /> < path d = "M3 7.8V3m0 0h4.8M3 3l6 6" /> </ svg > } }