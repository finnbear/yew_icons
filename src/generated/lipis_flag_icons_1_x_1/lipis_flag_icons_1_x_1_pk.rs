use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_pk (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-pk" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "pk-a" > < path fill - opacity = ".7" d = "M0 0h512v512H0z" /> </ clippath > </ defs > < g fill - rule = "evenodd" stroke - width = "1pt" clip - path = "url(#pk-a)" > < path fill = "#0c590b" d = "M-95 0h768v512H-95z" /> < path fill = "#fff" d = "M-95 0H97.5v512H-95z" /> < g fill = "#fff" > < path d = "m403.7 225.4-31.2-6.6-16.4 27.3-3.4-31.6-31-7.2 29-13-2.7-31.7 21.4 23.6 29.3-12.4-15.9 27.6 21 24z" /> < path d = "M415.4 306a121.2 121.2 0 0 1-161.3 59.4 122.1 122.1 0 0 1-59.5-162.1A118.6 118.6 0 0 1 266 139a156.2 156.2 0 0 0-11.8 10.9A112.3 112.3 0 0 0 415.5 306z" /> </ g > </ g > </ svg > } }