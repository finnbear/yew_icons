use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_lines_leaning (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 384 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M62.36 458.1C56.77 474.9 38.65 483.9 21.88 478.4C5.116 472.8-3.946 454.6 1.643 437.9L129.6 53.88C135.2 37.12 153.4 28.05 170.1 33.64C186.9 39.23 195.9 57.35 190.4 74.12L62.36 458.1zM261.3 32.44C278.7 35.34 290.5 51.83 287.6 69.26L223.6 453.3C220.7 470.7 204.2 482.5 186.7 479.6C169.3 476.7 157.5 460.2 160.4 442.7L224.4 58.74C227.3 41.31 243.8 29.53 261.3 32.44H261.3zM352 32C369.7 32 384 46.33 384 64V448C384 465.7 369.7 480 352 480C334.3 480 320 465.7 320 448V64C320 46.33 334.3 32 352 32V32z" /></ svg > } }