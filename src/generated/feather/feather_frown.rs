use crate :: IconProps ; # [inline (never)] pub fn feather_frown (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < circle cx = "12" cy = "12" r = "10" /> < path d = "M16 16s-1.5-2-4-2-4 2-4 2" /> < line x1 = "9" y1 = "9" x2 = "9.01" y2 = "9" /> < line x1 = "15" y1 = "9" x2 = "15.01" y2 = "9" /> </ svg > } }