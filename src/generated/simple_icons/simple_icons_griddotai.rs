use crate :: IconProps ; # [inline (never)] pub fn simple_icons_griddotai (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M17.732 9.091v-3.52H6.506v12.816h5.612v-5.613h11.226V24h-5.613v-5.613H12.12V24h-4.5a6.965 6.965 0 0 1-6.964-6.964V6.966A6.966 6.966 0 0 1 7.619 0h8.762a6.965 6.965 0 0 1 6.964 6.964v2.127h-5.613z" /></ svg > } }