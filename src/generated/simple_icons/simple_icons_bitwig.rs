use crate :: IconProps ; # [inline (never)] pub fn simple_icons_bitwig (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M4.15 7.782a1.59 1.59 0 1 1 3.181 0 1.59 1.59 0 0 1-3.181 0zm5.741 1.591a1.59 1.59 0 1 0 0-3.181 1.59 1.59 0 0 0 0 3.181zm4.218 0a1.59 1.59 0 1 0 0-3.181 1.59 1.59 0 0 0 0 3.181zm4.15 0a1.59 1.59 0 1 0 0-3.181 1.59 1.59 0 0 0 0 3.181zM5.741 10.409a1.59 1.59 0 1 0 0 3.181 1.59 1.59 0 0 0 0-3.181zm8.368 0a1.59 1.59 0 1 0 0 3.181 1.59 1.59 0 0 0 0-3.181zm4.15 0a1.59 1.59 0 1 0 0 3.181 1.59 1.59 0 0 0 0-3.181zm4.15 3.182a1.59 1.59 0 1 0 0-3.181 1.59 1.59 0 0 0 0 3.181zM1.591 10.409a1.591 1.591 0 1 0 0 3.182 1.591 1.591 0 0 0 0-3.182zm4.15 4.218a1.59 1.59 0 1 0 0 3.181 1.59 1.59 0 0 0 0-3.181zm12.518 0a1.59 1.59 0 1 0 0 3.181 1.59 1.59 0 0 0 0-3.181zm4.15 0a1.59 1.59 0 1 0 0 3.181 1.59 1.59 0 0 0 0-3.181zm-20.818 0a1.59 1.59 0 1 0 0 3.181 1.59 1.59 0 0 0 0-3.181m8.3-4.218a1.591 1.591 0 1 0 0 3.182 1.591 1.591 0 0 0 0-3.182Z" /></ svg > } }