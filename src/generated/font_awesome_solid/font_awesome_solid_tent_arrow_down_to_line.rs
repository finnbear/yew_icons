use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_tent_arrow_down_to_line (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M241.8 111.9C250.7 121.8 249.9 136.1 240.1 145.8L160.1 217.8C150.9 226.1 137.1 226.1 127.9 217.8L47.94 145.8C38.09 136.1 37.29 121.8 46.16 111.9C55.03 102.1 70.2 101.3 80.05 110.2L119.1 146.1V24C119.1 10.75 130.7 0 143.1 0C157.3 0 167.1 10.75 167.1 24V146.1L207.9 110.2C217.8 101.3 232.1 102.1 241.8 111.9H241.8zM364.6 134.5C376.1 125.8 391.9 125.8 403.4 134.5L571.4 262.5C578 267.6 582.4 275 583.6 283.3L608.4 448C625.9 448.2 640 462.4 640 480C640 497.7 625.7 512 608 512H32C14.33 512 0 497.7 0 480C0 462.3 14.33 448 32 448H159.6L184.4 283.3C185.6 275 189.1 267.6 196.6 262.5L364.6 134.5zM384 448H460.8L384 320V448z" /></ svg > } }