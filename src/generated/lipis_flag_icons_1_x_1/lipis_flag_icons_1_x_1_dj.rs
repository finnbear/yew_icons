use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_dj (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-dj" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "dj-a" > < path fill - opacity = ".7" d = "M55.4 0H764v708.7H55.4z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "translate(-40) scale(.722)" > < path fill = "#0c0" d = "M0 0h1063v708.7H0z" /> < path fill = "#69f" d = "M0 0h1063v354.3H0z" /> < path fill = "#fffefe" d = "m0 0 529.7 353.9L0 707.3V0z" /> < path fill = "red" d = "m221.2 404.3-42.7-30.8-42.4 31 15.8-50.3-42.4-31.2 52.4-.4 16.3-50.2 16.6 50 52.4.2-42.1 31.4 16 50.3z" /> </ g > </ svg > } }