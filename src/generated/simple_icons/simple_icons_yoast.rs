use crate :: IconProps ; # [inline (never)] pub fn simple_icons_yoast (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M15.864 0L14.55 3.652H5.422A3.592 3.592 0 0 0 1.84 7.233v9.529a3.592 3.592 0 0 0 3.582 3.581h1.495a4.9 4.9 0 0 1-.18.029l-.34.047V24h.391c2.76 0 4.442-1.385 5.706-3.657h9.666V7.233a3.593 3.593 0 0 0-3.253-3.565L20.275 0zm.556.778h2.738l-6.055 16.22c-1.55 4.335-3.186 6.064-5.924 6.21v-2.12c1.767-.354 2.418-1.461 2.785-2.408a3.902 3.902 0 0 0 0-2.828L6.43 6.772h2.488l2.512 7.86z" /></ svg > } }