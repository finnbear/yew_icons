use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_spoon (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M449.5 242.2C436.4 257.8 419.8 270 400.1 277.8C382.2 285.6 361.7 288.8 341.4 287C326.2 284.5 311.8 278.4 299.5 269.1L68.29 500.3C60.79 507.8 50.61 512 40 512C29.39 512 19.22 507.8 11.71 500.3C4.211 492.8-.0039 482.6-.0039 472C-.0039 461.4 4.211 451.2 11.71 443.7L243 212.5C233.7 200.2 227.6 185.8 225.1 170.6C223.3 150.3 226.5 129.9 234.3 111C242.1 92.22 254.3 75.56 269.9 62.47C337.8-5.437 433.1-20.28 482.7 29.35C532.3 78.95 517.4 174.2 449.5 242.2z" /></ svg > } }