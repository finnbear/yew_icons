use crate :: IconProps ; # [inline (never)] pub fn simple_icons_affinitypublisher (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M10.44 0L9.36 1.87 22.136 24h1.144a.72.72 0 00.72-.72v-3.119L12.36 0zm3 0L24 18.29V.72a.72.72 0 00-.72-.72zM8.82 2.806l-1.98 3.43L16.976 24h4.08zM6.24 7.274L4.76 9.837a1.941 1.941 0 000 1.942C6.531 14.842 11.816 24 11.816 24h4.08zm-2.58 4.47L0 18.082v5.197c0 .397.323.72.72.72h10.016Z" /></ svg > } }