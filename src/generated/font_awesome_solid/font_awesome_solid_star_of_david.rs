use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_star_of_david (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M490.7 345.4L435.6 256l55.1-89.38c14.87-24.25-3.62-54.61-33.12-54.61l-110.6-.005l-57.87-93.1C281.7 6.003 268.9 0 256 0C243.1 0 230.3 6.003 222.9 18L165 112H54.39c-29.62 0-47.99 30.37-33.12 54.62L76.37 256l-55.1 89.38C6.4 369.6 24.77 399.1 54.39 399.1h110.6l57.87 93.1C230.3 505.1 243.1 512 256 512c12.88 0 25.74-6.002 33.12-18l57.83-93.1h110.7C487.2 399.1 505.6 369.6 490.7 345.4zM256 73.77l23.59 38.23H232.5L256 73.77zM89.48 343.1l20.59-33.35l20.45 33.35H89.48zM110 201.3L89.48 168h41.04L110 201.3zM256 438.2l-23.59-38.25h47.08L256 438.2zM313.9 343.1H198L143.8 256l54.22-87.1h116L368.3 256L313.9 343.1zM381.3 343.1l20.67-33.29l20.52 33.29H381.3zM401.1 201.3l-20.51-33.29h41.04L401.1 201.3z" /></ svg > } }