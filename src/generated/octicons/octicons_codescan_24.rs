use crate :: IconProps ; # [inline (never)] pub fn octicons_codescan_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11.97 6.97a.75.75 0 000 1.06l2.47 2.47-2.47 2.47a.75.75 0 101.06 1.06l3-3a.75.75 0 000-1.06l-3-3a.75.75 0 00-1.06 0zM9.03 8.03a.75.75 0 00-1.06-1.06l-3 3a.75.75 0 000 1.06l3 3a.75.75 0 001.06-1.06L6.56 10.5l2.47-2.47z" />< path fill - rule = "evenodd" d = "M10.5 0C4.701 0 0 4.701 0 10.5S4.701 21 10.5 21c2.63 0 5.033-.967 6.875-2.564l4.345 4.344a.75.75 0 101.06-1.06l-4.344-4.345A10.459 10.459 0 0021 10.5C21 4.701 16.299 0 10.5 0zm-9 10.5a9 9 0 1118 0 9 9 0 01-18 0z" /></ svg > } }