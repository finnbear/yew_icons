use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_cm (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-cm" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#007a5e" d = "M0 0h170.7v512H0z" /> < path fill = "#ce1126" d = "M170.7 0h170.6v512H170.7z" /> < path fill = "#fcd116" d = "M341.3 0H512v512H341.3z" /> < g fill = "#fcd116" transform = "translate(256 256) scale(5.6889)" > < g id = "b" > < path id = "a" d = "M0-8-2.5-.4 1.3.9z" /> < use href = "#a" width = "100%" height = "100%" transform = "scale(-1 1)" /> </ g > < use href = "#b" width = "100%" height = "100%" transform = "rotate(72)" /> < use href = "#b" width = "100%" height = "100%" transform = "rotate(144)" /> < use href = "#b" width = "100%" height = "100%" transform = "rotate(-144)" /> < use href = "#b" width = "100%" height = "100%" transform = "rotate(-72)" /> </ g > </ svg > } }