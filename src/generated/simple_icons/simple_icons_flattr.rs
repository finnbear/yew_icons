use crate :: IconProps ; # [inline (never)] pub fn simple_icons_flattr (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M1.906 12C1.906 6.432 6.432 1.906 12 1.906c.048 0 4.003 0 5.455.002L14.53 4.834l1.344 1.344L21.903 0H12C5.373 0 0 5.373 0 12v9.331l1.91-1.759v-.096c-.002-.244-.004-7.404-.004-7.476zM24 2.668l-1.91 1.76v.096L22.093 12c0 5.568-4.528 10.094-10.093 10.094-.048 0-4.003 0-5.455-.002l2.925-2.926-1.344-1.344L2.097 24H12c6.627 0 12-5.373 12-12V2.668z" /></ svg > } }