use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_kw (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-kw" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "kw-a" > < path fill - opacity = ".7" d = "M0 0h682.7v512H0z" /> </ clippath > </ defs > < g fill - rule = "evenodd" stroke - width = "1pt" transform = "scale(.9375)" > < path fill = "#fff" d = "M0 170.6h1024v170.7H0z" /> < path fill = "#f31830" d = "M0 341.3h1024V512H0z" /> < path fill = "#00d941" d = "M0 0h1024v170.7H0z" /> < path d = "M0 0v512l255.4-170.7.6-170.8L0 0z" /> </ g > </ svg > } }