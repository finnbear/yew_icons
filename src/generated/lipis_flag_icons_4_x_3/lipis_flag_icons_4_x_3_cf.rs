use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_cf (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-cf" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "cf-a" > < path fill - opacity = ".7" d = "M-12.4 32h640v480h-640z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "translate(12.4 -32)" > < path fill = "#00f" d = "M-52 32h719.3v119H-52z" /> < path fill = "#ff0" d = "M-52 391.6h719.3V512H-52z" /> < path fill = "#009a00" d = "M-52 271.3h719.3v120.3H-52z" /> < path fill = "#fff" d = "M-52 151h719.3v120.3H-52z" /> < path fill = "red" d = "M247.7 32.5h119.9V512H247.7z" /> < path fill = "#ff0" d = "m99.3 137.7-31.5-21.8-31.3 22L47.4 101 16.9 78l38.2-1 12.5-36.3L80.3 77l38.1.7L88.2 101" /> </ g > </ svg > } }