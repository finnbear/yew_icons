use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_bahai (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M496.3 202.5l-110-15.38l41.88-104.4c6.625-16.63-11.63-32.25-26.63-22.63L307.5 120l-34.13-107.1C270.6 4.25 263.4 0 255.1 0C248.6 0 241.4 4.25 238.6 12.88L204.5 120L110.5 60.12c-15-9.5-33.22 5.1-26.6 22.63l41.85 104.4L15.71 202.5C-1.789 205-5.915 228.8 9.71 237.2l98.14 52.63l-74.51 83.5c-10.88 12.25-1.78 31 13.35 31c1.25 0 2.657-.25 4.032-.5l108.6-23.63l-4.126 112.5C154.7 504.4 164.1 512 173.6 512c5.125 0 10.38-2.25 14.25-7.25l68.13-88.88l68.23 88.88C327.1 509.8 333.2 512 338.4 512c9.5 0 18.88-7.625 18.38-19.25l-4.032-112.5l108.5 23.63c17.38 3.75 29.25-17.25 17.38-30.5l-74.51-83.5l98.14-52.72C517.9 228.8 513.8 205 496.3 202.5zM338.5 311.6L286.6 300.4l2 53.75l-32.63-42.5l-32.63 42.5l2-53.75L173.5 311.6l35.63-39.87L162.1 246.6L214.7 239.2L194.7 189.4l45 28.63L255.1 166.8l16.25 51.25l45-28.63L297.2 239.2l52.63 7.375l-47 25.13L338.5 311.6z" /></ svg > } }