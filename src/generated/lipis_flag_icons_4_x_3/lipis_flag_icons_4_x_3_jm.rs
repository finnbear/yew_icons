use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_jm (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-jm" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" > < path d = "m0 0 320 240L0 480zm640 0L320 240l320 240z" /> < path fill = "#090" d = "m0 0 320 240L640 0zm0 480 320-240 320 240z" /> < path fill = "#fc0" d = "M640 0h-59.6L0 435.3V480h59.6L640 44.7z" /> < path fill = "#fc0" d = "M0 0v44.7L580.4 480H640v-44.7L59.6 0z" /> </ g > </ svg > } }