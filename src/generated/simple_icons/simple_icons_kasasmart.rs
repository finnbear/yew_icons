use crate :: IconProps ; # [inline (never)] pub fn simple_icons_kasasmart (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12 0c-.5 0-1 .25-1.5.75L7.97 3.28l8.83 8.83c1 1 1.5 2 1.5 3V24h3.3c1.6 0 2.4-.8 2.4-2.4v-8.85c0-1-.5-2-1.5-3l-9-9C13 .25 12.5 0 12 0zM6.9 4.34L2.89 8.37 9.6 15.1c1 1 1.5 2 1.5 3V24h5.7v-8.89c-.03-.83-.6-1.46-1.06-1.94L6.91 4.34zm-5.08 5.1l-.32.31c-1 1-1.5 2-1.5 3v8.85C0 23.2.8 24 2.4 24h7.2v-5.9c-.03-.8-.56-1.42-1.06-1.95Z" /></ svg > } }