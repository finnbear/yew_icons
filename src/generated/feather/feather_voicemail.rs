use crate :: IconProps ; # [inline (never)] pub fn feather_voicemail (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > if let Some (title) = title . clone () { < title > { title } </ title > } < circle cx = "5.5" cy = "11.5" r = "4.5" /> < circle cx = "18.5" cy = "11.5" r = "4.5" /> < line x1 = "5.5" y1 = "16" x2 = "18.5" y2 = "16" /> </ svg > } }