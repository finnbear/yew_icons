use crate :: IconProps ; # [inline (never)] pub fn simple_icons_airasia (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11.54 14.49c-1.278 0-2.264-.998-2.264-2.276 0-1.252.98-2.27 2.264-2.27 1.232 0 2.238 1.018 2.238 2.27 0 1.278-1.005 2.277-2.239 2.277zm3.074-7.854l-.214.998c-.59-1.18-2.348-1.297-3.295-1.297-2.952 0-5.527 2.841-5.527 6.746 0 3.14 1.875 5.111 4.23 5.111 1.316 0 2.432-.304 3.353-1.4l-.24 1.102h3.711l1.692-11.26c-1.238-.001-2.482.01-3.71 0zM12 0c6.63 0 12 5.37 12 12s-5.37 12-12 12S0 18.63 0 12 5.37 0 12 0Z" /></ svg > } }