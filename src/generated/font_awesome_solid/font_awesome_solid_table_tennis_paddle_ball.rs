use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_table_tennis_paddle_ball (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M416 287.1c27.99 0 53.68 9.254 74.76 24.51c14.03-29.82 21.06-62.13 21.06-94.43c0-103.1-79.37-218.1-216.5-218.1c-59.94 0-120.4 23.71-165.5 68.95l-54.66 54.8C73.61 125.3 72.58 126.1 71.14 128.5l230.7 230.7C322.8 317.2 365.8 287.1 416 287.1zM290.3 392.1l-238.6-238.6C38.74 176.2 32.3 199.4 32.3 221.9c0 30.53 11.71 59.94 34.29 82.58l36.6 36.7l-92.38 81.32c-7.177 6.255-10.81 15.02-10.81 23.81c0 8.027 3.032 16.07 9.164 22.24l34.05 34.2c6.145 6.16 14.16 9.205 22.15 9.205c8.749 0 17.47-3.649 23.7-10.86l81.03-92.85l35.95 36.04c23.62 23.68 54.41 35.23 85.37 35.23c4.532 0 9.205-.2677 13.72-.7597c-10.56-18.61-17.12-39.89-17.12-62.81C288 408.1 288.1 400.5 290.3 392.1zM415.1 320c-52.99 0-95.99 42.1-95.99 95.1c0 52.1 42.99 95.99 95.99 95.99c52.1 0 95.99-42.1 95.99-95.99C511.1 363 468.1 320 415.1 320z" /></ svg > } }