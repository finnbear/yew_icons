use crate :: IconProps ; # [inline (never)] pub fn heroicons_solid_device_tablet (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/tailwindlabs/heroicons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 24 24" fill = "currentColor" aria - hidden = "true" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M10.5 18a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z" /> < path fill - rule = "evenodd" d = "M7.125 1.5A3.375 3.375 0 003.75 4.875v14.25A3.375 3.375 0 007.125 22.5h9.75a3.375 3.375 0 003.375-3.375V4.875A3.375 3.375 0 0016.875 1.5h-9.75zM6 4.875c0-.621.504-1.125 1.125-1.125h9.75c.621 0 1.125.504 1.125 1.125v14.25c0 .621-.504 1.125-1.125 1.125h-9.75A1.125 1.125 0 016 19.125V4.875z" clip - rule = "evenodd" /> </ svg > } }