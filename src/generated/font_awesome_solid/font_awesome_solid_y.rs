use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_y (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 384 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M378 82.61L224 298.3v149.8c0 17.67-14.31 31.1-32 31.1S160 465.7 160 448V298.3L5.969 82.61C-4.313 68.23-.9688 48.25 13.41 37.97c14.34-10.27 34.38-6.922 44.63 7.453L192 232.1l133.1-187.5c10.28-14.37 30.28-17.7 44.63-7.453C384.1 48.25 388.3 68.23 378 82.61z" /></ svg > } }