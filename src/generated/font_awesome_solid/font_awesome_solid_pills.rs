use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_pills (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M112 32C50.12 32 0 82.12 0 143.1v223.1c0 61.88 50.12 111.1 112 111.1s112-50.12 112-111.1V143.1C224 82.12 173.9 32 112 32zM160 256H64V144c0-26.5 21.5-48 48-48s48 21.5 48 48V256zM299.8 226.2c-3.5-3.5-9.5-3-12.38 .875c-45.25 62.5-40.38 150.1 15.88 206.4c56.38 56.25 144 61.25 206.5 15.88c4-2.875 4.249-8.75 .75-12.25L299.8 226.2zM529.5 207.2c-56.25-56.25-143.9-61.13-206.4-15.87c-4 2.875-4.375 8.875-.875 12.38l210.9 210.7c3.5 3.5 9.375 3.125 12.25-.75C590.8 351.1 585.9 263.6 529.5 207.2z" /></ svg > } }