use crate :: IconProps ; # [inline (never)] pub fn simple_icons_powerautomate (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "m20.079 7.235 3.666 4.097c.34.38.34.956 0 1.336l-7.63 8.528c-.19.213-.462.334-.748.334H7.289l12.79-14.295ZM6.789 21.53H1.005c-.867 0-1.326-1.025-.748-1.671L15.748 2.545c.139.057.265.145.367.259l3.631 4.058a.478.478 0 0 0-.039.039L6.916 21.197a.497.497 0 0 0-.127.333Zm8.356-19.06-8.192 9.155L.257 4.141c-.578-.646-.119-1.671.748-1.671h14.14Z" /></ svg > } }