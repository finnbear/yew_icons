use crate :: IconProps ; # [inline (never)] pub fn octicons_arrow_down_left_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M5.75 8.5a.75.75 0 00-.75.75v9c0 .414.336.75.75.75h9a.75.75 0 000-1.5H7.56L17.78 7.28a.75.75 0 00-1.06-1.06L6.5 16.44V9.25a.75.75 0 00-.75-.75z" /></ svg > } }