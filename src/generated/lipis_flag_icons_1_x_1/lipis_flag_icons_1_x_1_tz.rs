use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_tz (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-tz" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "tz-a" > < path fill - opacity = ".7" d = "M102.9 0h496v496H103z" /> </ clippath > </ defs > < g transform = "translate(-106.2) scale(1.0321)" > < g fill - rule = "evenodd" stroke - width = "1pt" > < path fill = "#09f" d = "M0 0h744.1v496H0z" /> < path fill = "#090" d = "M0 0h744.1L0 496V0z" /> < path d = "M0 496h165.4L744 103.4V0H578.7L0 392.7v103.4z" /> < path fill = "#ff0" d = "M0 378 567 0h56L0 415.3v-37.2zm121.1 118 623-415.3V118L177 496h-55.9z" /> </ g > </ g > </ svg > } }