use crate :: IconProps ; # [inline (never)] pub fn bootstrap_modem_fill (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/twbs/icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M7 0a1.5 1.5 0 0 0-1.5 1.5v11a1.5 1.5 0 0 0 1.404 1.497c-.35.305-.872.678-1.628 1.056A.5.5 0 0 0 5.5 16h5a.5.5 0 0 0 .224-.947c-.756-.378-1.278-.75-1.628-1.056A1.5 1.5 0 0 0 10.5 12.5v-11A1.5 1.5 0 0 0 9 0H7Zm1 3a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1Zm0 2a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1Zm.5 1.5a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0ZM8 9a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1Z" /> </ svg > } }