use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_is (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-is" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "is-a" > < path fill - opacity = ".7" d = "M0 0h640v480H0z" /> </ clippath > </ defs > < g fill - rule = "evenodd" stroke - width = "0" clip - path = "url(#is-a)" > < path fill = "#003897" d = "M0 0h666.7v480H0z" /> < path fill = "#fff" d = "M0 186.7h186.7V0h106.6v186.7h373.4v106.6H293.3V480H186.7V293.3H0V186.7z" /> < path fill = "#d72828" d = "M0 213.3h213.3V0h53.4v213.3h400v53.4h-400V480h-53.4V266.7H0v-53.4z" /> </ g > </ svg > } }