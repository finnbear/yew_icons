use crate :: IconProps ; # [inline (never)] pub fn lucide_cog (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z" /> < path d = "M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z" /> < path d = "M12 2v2" /> < path d = "M12 22v-2" /> < path d = "m17 20.66-1-1.73" /> < path d = "M11 10.27 7 3.34" /> < path d = "m20.66 17-1.73-1" /> < path d = "m3.34 7 1.73 1" /> < path d = "M14 12h8" /> < path d = "M2 12h2" /> < path d = "m20.66 7-1.73 1" /> < path d = "m3.34 17 1.73-1" /> < path d = "m17 3.34-1 1.73" /> < path d = "m11 13.73-4 6.93" /> </ svg > } }