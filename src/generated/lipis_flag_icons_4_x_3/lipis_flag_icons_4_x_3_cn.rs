use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_cn (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-cn" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < path id = "a" fill = "#ffde00" d = "M-.6.8 0-1 .6.8-1-.3h2z" /> </ defs > < path fill = "#de2910" d = "M0 0h640v480H0z" /> < use href = "#a" width = "30" height = "20" transform = "matrix(71.9991 0 0 72 120 120)" /> < use href = "#a" width = "30" height = "20" transform = "matrix(-12.33562 -20.5871 20.58684 -12.33577 240.3 48)" /> < use href = "#a" width = "30" height = "20" transform = "matrix(-3.38573 -23.75998 23.75968 -3.38578 288 95.8)" /> < use href = "#a" width = "30" height = "20" transform = "matrix(6.5991 -23.0749 23.0746 6.59919 288 168)" /> < use href = "#a" width = "30" height = "20" transform = "matrix(14.9991 -18.73557 18.73533 14.99929 240 216)" /> </ svg > } }