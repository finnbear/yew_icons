use crate :: IconProps ; # [inline (never)] pub fn simple_icons_zorin (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M4 18.944L5.995 22.4h12.01L20 18.944H4zM24 12l-2.013 3.488H9.216l12.771-6.976L24 12zM0 12l2.013-3.488h12.771L2.013 15.488 0 12zm4-6.944L5.995 1.6h12.01L20 5.056H4z" /></ svg > } }