use crate :: IconProps ; # [inline (never)] pub fn simple_icons_verdaccio (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M17.376 9.84L18.72 7.2h-4.8v.566h.864l-.192.377H12.96v.566h1.344l-.288.565H12v.566h1.728zm-4.255 8.64l3.68-7.265h-3.68l-1.064 2.103L8.959 7.2H5.28l5.712 11.28zM8.88 0h6.24A8.86 8.86 0 0124 8.88v6.24A8.86 8.86 0 0115.12 24H8.88A8.86 8.86 0 010 15.12V8.88A8.86 8.86 0 018.88 0z" /></ svg > } }