use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_ps (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-ps" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "ps-a" > < path fill - opacity = ".7" d = "M-118 0h682.7v512H-118z" /> </ clippath > </ defs > < g transform = "translate(110.6) scale(.9375)" > < g fill - rule = "evenodd" stroke - width = "1pt" > < path d = "M-246 0H778v170.7H-246z" /> < path fill = "#fff" d = "M-246 170.7H778v170.6H-246z" /> < path fill = "#090" d = "M-246 341.3H778V512H-246z" /> < path fill = "red" d = "m-246 512 512-256L-246 0v512z" /> </ g > </ g > </ svg > } }