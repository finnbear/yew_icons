use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_gy (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-gy" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < g fill - rule = "evenodd" > < path fill = "#399408" d = "M2.4 0H640v480H2.4z" /> < path fill = "#fff" d = "M.2 0c-.9 0 619.6 241.5 619.6 241.5L0 479.8.2 0z" /> < path fill = "#ffde08" d = "M.3 20.2c3.4 0 559 217.9 555.9 220L1.9 463.2.3 20.3z" /> < path d = "M1.9.8c1.8 0 290.9 240.9 290.9 240.9L1.8 477V.8z" /> < path fill = "#de2110" d = "M.3 33.9c1.6-15 260.9 208.4 260.9 208.4L.2 451.7V33.9z" /> </ g > </ svg > } }