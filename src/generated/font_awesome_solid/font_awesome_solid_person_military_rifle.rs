use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_person_military_rifle (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 448 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M128 39C128 25.1 137.1 15.17 150.9 14.09L302.7 1.443C312 .6658 320 8.027 320 17.39V47.1C320 56.84 312.8 63.1 304 63.1H152.1C139.2 63.1 128 52.81 128 39V39zM302.4 95.1C303.5 101.2 304 106.5 304 111.1C304 156.2 268.2 191.1 224 191.1C179.8 191.1 144 156.2 144 111.1C144 106.5 144.6 101.2 145.6 95.1H302.4zM373.6 460.3L320 369.7V480C320 481.3 319.9 482.5 319.8 483.8L145.5 234.9C162.1 227.8 180.2 223.1 198.8 223.1H249.2C265.1 223.1 280.6 226.8 295 231.9L389.9 67.71C382.2 63.3 379.6 53.51 384 45.86C388.4 38.21 398.2 35.58 405.9 40L433.6 56C441.2 60.42 443.8 70.21 439.4 77.86L383.1 173.9L385.6 174.9C400.9 183.7 406.1 203.3 397.3 218.6L360.6 282C362.6 284.9 364.5 287.8 366.3 290.8L442.4 419.7C453.7 438.7 447.4 463.2 428.4 474.4C409.3 485.7 384.8 479.4 373.6 460.3V460.3zM264 319.1C277.3 319.1 288 309.3 288 295.1C288 282.7 277.3 271.1 264 271.1C250.7 271.1 240 282.7 240 295.1C240 309.3 250.7 319.1 264 319.1zM160 512C142.3 512 128 497.7 128 480V369.7L74.44 460.3C63.21 479.4 38.68 485.7 19.66 474.4C.6381 463.2-5.669 438.7 5.569 419.7L81.7 290.8C91.06 274.1 103.4 261.5 117.7 250.8L299.1 510C295.6 511.3 291.9 512 288 512L160 512z" /></ svg > } }