use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_not_equal (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 448 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M432 336c0 17.69-14.31 32.01-32 32.01H187.8l-65.15 97.74C116.5 474.1 106.3 480 95.97 480c-6.094 0-12.25-1.75-17.72-5.375c-14.72-9.812-18.69-29.66-8.875-44.38l41.49-62.23H48c-17.69 0-32-14.32-32-32.01s14.31-31.99 32-31.99h105.5l63.1-96H48c-17.69 0-32-14.32-32-32.01s14.31-31.99 32-31.99h212.2l65.18-97.77c9.781-14.69 29.62-18.66 44.37-8.875c14.72 9.812 18.69 29.66 8.875 44.38l-41.51 62.27H400c17.69 0 32 14.31 32 31.99s-14.31 32.01-32 32.01h-105.6l-63.1 96H400C417.7 304 432 318.3 432 336z" /></ svg > } }