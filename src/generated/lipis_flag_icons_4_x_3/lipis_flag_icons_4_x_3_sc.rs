use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_sc (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-sc" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "sc-a" > < path fill - opacity = ".7" d = "M0 0h682.7v512H0z" /> </ clippath > </ defs > < g fill - rule = "evenodd" stroke - width = "1pt" transform = "scale(.9375)" > < path fill = "red" d = "M0 0h992.1v512H0z" /> < path fill = "#090" d = "m0 512 992.1-170.7V512H0z" /> < path fill = "#fff" d = "m0 512 992.1-341.3v170.6L0 512z" /> < path fill = "#009" d = "M0 512V0h330.7L0 512z" /> < path fill = "#ff0" d = "M0 512 330.7 0h330.7L0 512z" /> </ g > </ svg > } }