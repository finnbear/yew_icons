use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_9 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M160 32.01c-88.22 0-160 71.78-160 160c0 85.57 67.71 155.1 152.3 159.2l-64.65 76.06c-11.47 13.45-9.812 33.66 3.656 45.09c6 5.125 13.38 7.62 20.72 7.62c9.062 0 18.06-3.823 24.38-11.28C281.9 297.5 320 266.6 320 192C320 103.8 248.2 32.01 160 32.01zM160 288c-52.94 0-96-43.06-96-95.1s43.06-96 96-96s96 43.06 96 96S212.9 288 160 288z" /></ svg > } }