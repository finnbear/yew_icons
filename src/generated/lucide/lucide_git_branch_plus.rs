use crate :: IconProps ; # [inline (never)] pub fn lucide_git_branch_plus (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lucide-icons/lucide - Licensed under ISC" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M6 3v12" /> < path d = "M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" /> < path d = "M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" /> < path d = "M15 6a9 9 0 0 0-9 9" /> < path d = "M18 15v6" /> < path d = "M21 18h-6" /> </ svg > } }