use crate :: IconProps ; # [inline (never)] pub fn simple_icons_automattic (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M14.521 8.11a1.497 1.497 0 01.433 2.102l-3.511 5.441a1.496 1.496 0 01-2.068.457 1.507 1.507 0 01-.44-2.08l3.513-5.44c.215-.335.554-.57.943-.655.39-.085.796-.04 1.13.175z M11.98 23.03C4.713 23.03 0 17.79 0 12.338v-.676C0 6.117 4.713.97 11.98.97 19.246.97 24 6.117 24 11.662v.676c0 5.453-4.713 10.692-12.02 10.692zm8.133-11.31c0-3.974-2.888-7.51-8.133-7.51-5.245 0-8.087 3.542-8.087 7.51v.497c0 3.974 2.888 7.578 8.087 7.578 5.198 0 8.133-3.604 8.133-7.578v-.497z" /></ svg > } }