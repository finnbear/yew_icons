use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_sy (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-sy" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < rect width = "512" height = "170.7" y = "170.7" fill = "#fff" fill - rule = "evenodd" rx = "0" ry = "0" /> < rect width = "512" height = "170.7" y = "341.3" fill - rule = "evenodd" rx = "0" ry = "0" /> < path fill = "red" fill - rule = "evenodd" d = "M0 0h512v170.7H0z" /> < path fill = "#090" fill - rule = "evenodd" d = "m151.4 299.7-30.8-22.2L90 300l11.4-36.6L70.9 241l37.8-.3 11.7-36.5 12 36.4H170l-30.4 22.8 11.7 36.4zm285.4 0-30.7-22.2-30.6 22.5 11.4-36.6-30.5-22.5 37.7-.3 11.7-36.5 12 36.4h37.8l-30.4 22.8 11.6 36.4z" /> </ svg > } }