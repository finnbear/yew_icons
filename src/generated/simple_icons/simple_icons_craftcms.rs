use crate :: IconProps ; # [inline (never)] pub fn simple_icons_craftcms (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M21.474 0H2.526A2.516 2.516 0 0 0 0 2.526v18.948A2.516 2.516 0 0 0 2.526 24h18.948A2.534 2.534 0 0 0 24 21.474V2.526A2.516 2.516 0 0 0 21.474 0m-9.516 14.625c.786 0 1.628-.31 2.442-1.039l1.123 1.291c-1.18.955-2.527 1.488-3.874 1.488-2.667 0-4.35-1.769-3.958-4.267.393-2.498 2.667-4.266 5.334-4.266 1.29 0 2.498.505 3.34 1.431l-1.572 1.291c-.45-.59-1.207-.982-2.05-.982-1.6 0-2.834 1.039-3.087 2.526-.224 1.488.674 2.527 2.302 2.527" /></ svg > } }