use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_person_harassing (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M144 48C144 21.49 165.5 0 192 0C218.5 0 240 21.49 240 48C240 74.51 218.5 96 192 96C165.5 96 144 74.51 144 48V48zM15.52 315.4C.3696 306.3-4.531 286.7 4.573 271.5L62.85 174.6C80.2 145.7 111.4 128 145.1 128H181C209.6 128 236.7 140.7 254.9 162.7L328.6 251.6C339.9 265.2 338 285.3 324.4 296.6C310.8 307.9 290.7 306 279.4 292.4L232 235.3V480C232 497.7 217.7 512 200 512C182.3 512 168 497.7 168 480V352H152V480C152 497.7 137.7 512 120 512C102.3 512 88 497.7 88 480V256.9L59.43 304.5C50.33 319.6 30.67 324.5 15.52 315.4H15.52zM480 240C480 266.5 458.5 288 432 288C405.5 288 384 266.5 384 240C384 213.5 405.5 192 432 192C458.5 192 480 213.5 480 240zM464 344C464 313.1 489.1 288 520 288C550.9 288 576 313.1 576 344V446.1C576 482.5 546.5 512 510.1 512C492.6 512 475.8 505.1 463.4 492.7L408.8 438L380.6 494.3C372.7 510.1 353.5 516.5 337.7 508.6C321.9 500.7 315.5 481.5 323.4 465.7L371.4 369.7C375.1 360.5 384.7 354.1 394.9 352.4C405 350.8 415.4 354.1 422.6 361.4L464 402.7V344zM288 48C288 39.16 295.2 32 304 32H360C368.8 32 376 39.16 376 48C376 56.84 368.8 64 360 64H304C295.2 64 288 56.84 288 48zM335.2 121.7C343.1 125.6 346.3 135.3 342.3 143.2C338.4 151.1 328.7 154.3 320.8 150.3L272.8 126.3C264.9 122.4 261.7 112.7 265.7 104.8C269.6 96.94 279.3 93.74 287.2 97.69L335.2 121.7z" /></ svg > } }