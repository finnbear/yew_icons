use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_cannabis (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M544 374.4c0 6-3.25 11.38-8.5 14.12c-2.5 1.375-60.75 31.75-133.5 31.75c-6.124 0-12-.125-17.5-.25c11.38 22.25 16.5 38.25 16.75 39.13c1.875 5.75 .375 12-3.875 16.12c-4.125 4.25-10.38 5.75-16.12 4c-1.631-.4648-32.94-10.66-69.25-34.06v42.81C312 501.3 301.3 512 288 512s-24-10.75-24-23.1v-42.81c-36.31 23.4-67.62 33.59-69.25 34.06c-5.75 1.75-12 .25-16.12-4c-4.25-4.25-5.75-10.38-3.875-16.12C175 458.3 180.1 442.1 191.5 420c-5.501 .125-11.37 .25-17.5 .25c-72.75 0-130.1-30.38-133.5-31.75C35.25 385.8 32 380.4 32 374.4c0-5.875 3.25-11.38 8.5-14.12c1.625-.875 32.38-16.88 76.75-25.75c-64.25-75.13-84-161.8-84.88-165.8C31.25 163.5 32.75 157.9 36.63 154C39.75 151 43.75 149.4 48 149.4c1.125 0 2.25 .125 3.375 .375C55.38 150.6 137.1 169.3 212 229.5V225.1c0-118.9 60-213.8 62.5-217.8C277.5 2.75 282.5 0 288 0s10.5 2.75 13.5 7.375C304 11.38 364 106.3 364 225.1V229.5c73.1-60.25 156.6-79 160.5-79.75C525.8 149.5 526.9 149.4 528 149.4c4.25 0 8.25 1.625 11.38 4.625c3.75 3.875 5.375 9.5 4.25 14.75c-.875 4-20.62 90.63-84.88 165.8c44.38 8.875 75.13 24.88 76.75 25.75C540.8 363 544 368.5 544 374.4z" /></ svg > } }