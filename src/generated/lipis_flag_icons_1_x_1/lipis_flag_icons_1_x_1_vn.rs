use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_vn (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-vn" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "vn-a" > < path fill - opacity = ".7" d = "M177.2 0h708.6v708.7H177.2z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "translate(-128) scale(.72249)" > < path fill = "#ec0015" d = "M0 0h1063v708.7H0z" /> < path fill = "#ff0" d = "m661 527.5-124-92.6-123.3 93.5 45.9-152-123.2-93.8 152.4-1.3L536 129.8 584.3 281l152.4.2-122.5 94.7L661 527.5z" /> </ g > </ svg > } }