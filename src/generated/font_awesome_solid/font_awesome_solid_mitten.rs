use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_mitten (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 448 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M351.1 416H63.99c-17.6 0-31.1 14.4-31.1 31.1l.0026 31.1C31.1 497.6 46.4 512 63.1 512h288c17.6 0 32-14.4 32-31.1l-.0049-31.1C383.1 430.4 369.6 416 351.1 416zM425 206.9c-27.25-22.62-67.5-19-90.13 8.25l-20.88 25L284.4 111.8c-18-77.5-95.38-125.1-172.8-108C34.26 21.63-14.25 98.88 3.754 176.4L64 384h288l81.14-86.1C455.8 269.8 452.1 229.5 425 206.9z" /></ svg > } }