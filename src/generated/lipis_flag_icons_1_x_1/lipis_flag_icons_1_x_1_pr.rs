use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_pr (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-pr" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "pr-a" > < path fill - opacity = ".7" d = "M51.6 0h708.7v708.7H51.6z" /> </ clippath > </ defs > < g fill - rule = "evenodd" transform = "translate(-37.3) scale(.72249)" > < path fill = "#ed0000" d = "M0 0h1063v708.7H0z" /> < path fill = "#fff" d = "M0 141.7h1063v141.8H0zm0 283.5h1063v141.7H0z" /> < path fill = "#0050f0" d = "m0 0 610 353.9L0 707.3V0z" /> < path fill = "#fff" d = "m268.2 450.5-65.7-49-65.3 49.5 24.3-80.5-65.3-49.7 80.7-.7 25-80.2 25.6 80 80.7.1-64.9 50.2 24.9 80.3z" /> </ g > </ svg > } }