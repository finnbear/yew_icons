use crate :: IconProps ; # [inline (never)] pub fn simple_icons_steamdeck (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M8.494 0v4.309c4.242 0 7.694 3.45 7.694 7.691s-3.452 7.691-7.694 7.691V24c6.617 0 12-5.383 12-12s-5.383-12-12-12zm10.819 3.62v.194h.328v.893h.228v-.893h.33V3.62zm1.037 0v1.087h.207v-.684l.298.653h.14l.29-.66v.691h.219V3.619h-.23l-.338.772-.368-.772zM8.494 6.011a5.998 5.998 0 0 0-5.998 6 5.998 5.998 0 1 0 11.998 0 6 6 0 0 0-6-6z" /></ svg > } }