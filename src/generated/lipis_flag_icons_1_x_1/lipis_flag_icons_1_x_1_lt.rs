use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_lt (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-lt" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" stroke - width = "1pt" transform = "scale(.51314 1.0322)" > < rect width = "1063" height = "708.7" fill = "#006a44" rx = "0" ry = "0" transform = "scale(.93865 .69686)" /> < rect width = "1063" height = "236.2" y = "475.6" fill = "#c1272d" rx = "0" ry = "0" transform = "scale(.93865 .69686)" /> < path fill = "#fdb913" d = "M0 0h997.8v164.6H0z" /> </ g > </ svg > } }