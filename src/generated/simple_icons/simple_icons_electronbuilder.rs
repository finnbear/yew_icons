use crate :: IconProps ; # [inline (never)] pub fn simple_icons_electronbuilder (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12 7.01a3.506 3.506 0 003.506-3.505A3.506 3.506 0 0012 0a3.506 3.506 0 00-3.506 3.506A3.506 3.506 0 0012 7.01m0 4.137C9.243 8.588 5.574 7.01 1.484 7.01v12.852C5.574 19.863 9.243 21.44 12 24c2.757-2.56 6.426-4.137 10.516-4.137V7.01c-4.09 0-7.759 1.578-10.516 4.137z" /></ svg > } }