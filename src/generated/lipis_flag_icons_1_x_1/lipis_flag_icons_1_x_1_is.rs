use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_is (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-is" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "is-a" > < path fill - opacity = ".7" d = "M85.4 0h486v486h-486z" /> </ clippath > </ defs > < g fill - rule = "evenodd" stroke - width = "0" transform = "translate(-90) scale(1.0535)" > < path fill = "#003897" d = "M0 0h675v486H0z" /> < path fill = "#fff" d = "M0 189h189V0h108v189h378v108H297v189H189V297H0V189z" /> < path fill = "#d72828" d = "M0 216h216V0h54v216h405v54H270v216h-54V270H0v-54z" /> </ g > </ svg > } }