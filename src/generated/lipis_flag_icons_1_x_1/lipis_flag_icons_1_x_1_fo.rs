use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_fo (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-fo" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "fo-a" > < path fill - opacity = ".7" d = "M0 0h512v512H0z" /> </ clippath > </ defs > < g fill - rule = "evenodd" stroke - width = "0" clip - path = "url(#fo-a)" > < path fill = "#fff" d = "M-78 0h708.2v512H-78z" /> < path fill = "#003897" d = "M-75.9 199.1h198.3V0h113.3v199.1h396.6V313H235.7v199H122.4V312.9H-76V199z" /> < path fill = "#d72828" d = "M-75.9 227.6h226.6V0h56.7v227.6h424.9v56.9h-425V512h-56.6V284.4H-75.9v-56.8z" /> </ g > </ svg > } }