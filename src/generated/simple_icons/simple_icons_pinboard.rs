use crate :: IconProps ; # [inline (never)] pub fn simple_icons_pinboard (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M13.352 14.585l-4.509 4.614.72-4.062L3.428 7.57 0 7.753 7.58 0v2.953l7.214 6.646 4.513-1.105-4.689 4.982L24 24l-10.648-9.415z" /></ svg > } }