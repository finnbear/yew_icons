use crate :: IconProps ; # [inline (never)] pub fn simple_icons_powerpages (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M5.42 14.624 3.585 16a1.258 1.258 0 0 0 0 2.014l7.66 5.745a1.257 1.257 0 0 0 1.51 0l2.612-1.959a1.841 1.841 0 0 1-.828-.337c-3.081-2.223-6.1-4.531-9.119-6.839Zm13.16-4.622 4.925 3.694c.66.503.66 1.497 0 2.001l-7.155 5.366a1.259 1.259 0 0 1-1.511 0l-5.693-4.27c.294-.038.58-.15.828-.337l8.606-6.454Zm-18.077.309a1.259 1.259 0 0 1 .001-2.014L11.245.241a1.257 1.257 0 0 1 1.51 0l7.661 5.745c.671.503.671 1.51 0 2.013L9.674 16.056a1.262 1.262 0 0 1-1.511 0l-7.66-5.745Z" /></ svg > } }