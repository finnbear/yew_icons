use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_st (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-st" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill = "#12ad2b" d = "M0 0h640v480H0z" /> < path fill = "#ffce00" d = "M0 137.1h640V343H0z" /> < path fill = "#d21034" d = "M0 0v480l240-240" /> < g id = "c" transform = "translate(351.6 240) scale(.34286)" > < g id = "b" > < path id = "a" d = "M0-200V0h100" transform = "rotate(18 0 -200)" /> < use href = "#a" width = "100%" height = "100%" transform = "scale(-1 1)" /> </ g > < use href = "#b" width = "100%" height = "100%" transform = "rotate(72)" /> < use href = "#b" width = "100%" height = "100%" transform = "rotate(144)" /> < use href = "#b" width = "100%" height = "100%" transform = "rotate(-144)" /> < use href = "#b" width = "100%" height = "100%" transform = "rotate(-72)" /> </ g > < use href = "#c" width = "100%" height = "100%" x = "700" transform = "translate(-523.2)" /> </ svg > } }