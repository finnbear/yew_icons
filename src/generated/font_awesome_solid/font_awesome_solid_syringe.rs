use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_syringe (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M504.1 71.03l-64-64c-9.375-9.375-24.56-9.375-33.94 0s-9.375 24.56 0 33.94L422.1 56L384 94.06l-55.03-55.03c-9.375-9.375-24.56-9.375-33.94 0c-8.467 8.467-8.873 21.47-2.047 30.86l149.1 149.1C446.3 222.1 451.1 224 456 224c6.141 0 12.28-2.344 16.97-7.031c9.375-9.375 9.375-24.56 0-33.94L417.9 128L456 89.94l15.03 15.03C475.7 109.7 481.9 112 488 112s12.28-2.344 16.97-7.031C514.3 95.59 514.3 80.41 504.1 71.03zM208.8 154.1l58.56 58.56c6.25 6.25 6.25 16.38 0 22.62C264.2 238.4 260.1 240 256 240S247.8 238.4 244.7 235.3L186.1 176.8L144.8 218.1l58.56 58.56c6.25 6.25 6.25 16.38 0 22.62C200.2 302.4 196.1 304 192 304S183.8 302.4 180.7 299.3L122.1 240.8L82.75 280.1C70.74 292.1 64 308.4 64 325.4v88.68l-56.97 56.97c-9.375 9.375-9.375 24.56 0 33.94C11.72 509.7 17.86 512 24 512s12.28-2.344 16.97-7.031L97.94 448h88.69c16.97 0 33.25-6.744 45.26-18.75l187.6-187.6l-149.1-149.1L208.8 154.1z" /></ svg > } }