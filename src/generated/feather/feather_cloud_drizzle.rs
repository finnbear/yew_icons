use crate :: IconProps ; # [inline (never)] pub fn feather_cloud_drizzle (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < line x1 = "8" y1 = "19" x2 = "8" y2 = "21" /> < line x1 = "8" y1 = "13" x2 = "8" y2 = "15" /> < line x1 = "16" y1 = "19" x2 = "16" y2 = "21" /> < line x1 = "16" y1 = "13" x2 = "16" y2 = "15" /> < line x1 = "12" y1 = "21" x2 = "12" y2 = "23" /> < line x1 = "12" y1 = "15" x2 = "12" y2 = "17" /> < path d = "M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25" /> </ svg > } }