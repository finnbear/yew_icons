use crate :: IconProps ; # [inline (never)] pub fn octicons_codescan_16 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M8.47 4.97a.75.75 0 000 1.06L9.94 7.5 8.47 8.97a.75.75 0 101.06 1.06l2-2a.75.75 0 000-1.06l-2-2a.75.75 0 00-1.06 0zM6.53 6.03a.75.75 0 00-1.06-1.06l-2 2a.75.75 0 000 1.06l2 2a.75.75 0 101.06-1.06L5.06 7.5l1.47-1.47z" />< path fill - rule = "evenodd" d = "M12.246 13.307a7.5 7.5 0 111.06-1.06l2.474 2.473a.75.75 0 11-1.06 1.06l-2.474-2.473zM1.5 7.5a6 6 0 1110.386 4.094.75.75 0 00-.292.293A6 6 0 011.5 7.5z" /></ svg > } }