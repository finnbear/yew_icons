use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_cn (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-cn" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < path id = "a" fill = "#ffde00" d = "M1-.3-.7.8 0-1 .6.8-1-.3z" /> </ defs > < path fill = "#de2910" d = "M0 0h512v512H0z" /> < use href = "#a" width = "30" height = "20" transform = "matrix(76.8 0 0 76.8 128 128)" /> < use href = "#a" width = "30" height = "20" transform = "rotate(-121 142.6 -47) scale(25.5827)" /> < use href = "#a" width = "30" height = "20" transform = "rotate(-98.1 198 -82) scale(25.6)" /> < use href = "#a" width = "30" height = "20" transform = "rotate(-74 272.4 -114) scale(25.6137)" /> < use href = "#a" width = "30" height = "20" transform = "matrix(16 -19.968 19.968 16 256 230.4)" /> </ svg > } }