use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_ss (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-ss" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#078930" d = "M0 358.4h512V512H0z" /> < path fill = "#fff" d = "M0 153.6h512v204.8H0z" /> < path d = "M0 0h512v153.6H0z" /> < path fill = "#da121a" d = "M0 179.2h512v153.6H0z" /> < path fill = "#0f47af" d = "m0 0 433 256L0 512z" /> < path fill = "#fcdd09" d = "M209 207.8 64.4 256l144.8 48.1-89.5-126v155.8z" /> </ svg > } }