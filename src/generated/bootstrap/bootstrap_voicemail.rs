use crate :: IconProps ; # [inline (never)] pub fn bootstrap_voicemail (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/twbs/icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M7 8.5A3.49 3.49 0 0 1 5.95 11h4.1a3.5 3.5 0 1 1 2.45 1h-9A3.5 3.5 0 1 1 7 8.5zm-6 0a2.5 2.5 0 1 0 5 0 2.5 2.5 0 0 0-5 0zm14 0a2.5 2.5 0 1 0-5 0 2.5 2.5 0 0 0 5 0z" /> </ svg > } }