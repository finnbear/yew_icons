use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_face_grin_squint (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM256.3 331.8C208.9 331.8 164.1 324.9 124.5 312.8C112.2 309 100.2 319.7 105.2 331.5C130.1 390.6 188.4 432 256.3 432C324.2 432 382.4 390.6 407.4 331.5C412.4 319.7 400.4 309 388.1 312.8C348.4 324.9 303.7 331.8 256.3 331.8H256.3zM133.5 146.7C125.6 142.4 116 148.2 116 157.1C116 159.9 116.1 162.6 118.8 164.8L154.8 208L118.8 251.2C116.1 253.4 116 256.1 116 258.9C116 267.8 125.6 273.6 133.5 269.3L223.4 221.4C234.1 215.7 234.1 200.3 223.4 194.6L133.5 146.7zM396 157.1C396 148.2 386.4 142.4 378.5 146.7L288.6 194.6C277.9 200.3 277.9 215.7 288.6 221.4L378.5 269.3C386.4 273.6 396 267.8 396 258.9C396 256.1 395 253.4 393.2 251.2L357.2 208L393.2 164.8C395 162.6 396 159.9 396 157.1V157.1z" /></ svg > } }