use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_shield_blank (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M496 127.1C496 381.3 309.1 512 255.1 512C204.9 512 16 385.3 16 127.1c0-19.41 11.7-36.89 29.61-44.28l191.1-80.01c4.906-2.031 13.13-3.701 18.44-3.701c5.281 0 13.58 1.67 18.46 3.701l192 80.01C484.3 91.1 496 108.6 496 127.1z" /></ svg > } }