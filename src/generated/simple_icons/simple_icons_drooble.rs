use crate :: IconProps ; # [inline (never)] pub fn simple_icons_drooble (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M24 11.986a7.599 7.599 0 0 0-7-7.559v7.574a5 5 0 0 1-10 0c0-3.604 3.707-6.022 7-4.583V.17C6.615-1.069 0 4.63 0 12c0 6.628 5.373 12 12 12 6.628 0 12-5.372 12-12v-.014m-14 .015a2 2 0 1 0 4 0 2 2 0 0 0-4 0m14-.015a7.599 7.599 0 0 0-7-7.559v7.574a5 5 0 0 1-10 0c0-3.604 3.707-6.022 7-4.583V.17C6.615-1.069 0 4.63 0 12c0 6.628 5.373 12 12 12 6.628 0 12-5.372 12-12v-.014m-14 .015a2 2 0 1 0 4 0 2 2 0 0 0-4 0Z" /></ svg > } }