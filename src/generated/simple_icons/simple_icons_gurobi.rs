use crate :: IconProps ; # [inline (never)] pub fn simple_icons_gurobi (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "m11.036 0 7.032 1.359L24 18.37 18.37 24 0 17.635 1.805 5.952 11.036 0Zm12.389 18.239L17.887 2.36l-3.557 7.83 3.88 13.264 5.215-5.214Zm-5.822-16.46L11.138.528l-8.71 5.617 11.554 3.6 3.62-7.968Z" /></ svg > } }