use crate :: IconProps ; # [inline (never)] pub fn simple_icons_aframe (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M17.37 17.07H6.57L4.24 24H3.01l8.23-24h1.52l8.23 24h-1.3zm-.39-1.13l-5-14.96-5.03 14.98h10.03Z" /></ svg > } }