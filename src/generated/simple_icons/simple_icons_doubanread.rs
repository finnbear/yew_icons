use crate :: IconProps ; # [inline (never)] pub fn simple_icons_doubanread (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M15.328 5.553c-2.648.906-4.008 4.372-7.101 4.833C4.827 10.833.752 7.205 0 6c0 0 .526.906 1.28 2.105C5.205 14.297 7.772 18.224 12 18.75c5.28.68 8.146-4.535 8.826-6.64.607-1.732 1.733-1.66 2.494-1.433l.68.227s-2.729-7.402-8.688-5.36l.016.008z" /></ svg > } }