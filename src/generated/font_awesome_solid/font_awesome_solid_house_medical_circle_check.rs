use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_house_medical_circle_check (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M320.5 381.5C324.6 435.5 353 482.6 394.8 512H128.1C92.75 512 64.09 483.3 64.09 448V287.6H32.05C14.02 287.6 0 273.5 0 255.5C0 246.5 3.004 238.5 10.01 231.5L266.4 8.016C273.4 1.002 281.4 0 288.4 0C295.4 0 303.4 2.004 309.5 7.014L522.1 193.9C513.6 192.7 504.9 192 496 192C453.6 192 414.7 207 384.3 232L384 232H328V176C328 167.2 320.8 160 311.1 160H263.1C255.2 160 247.1 167.2 247.1 176V232H191.1C183.2 232 175.1 239.2 175.1 248V296C175.1 304.8 183.2 312 191.1 312H247.1V368C247.1 376.8 255.2 384 263.1 384H311.1C315.1 384 318 383.1 320.5 381.5H320.5zM328 312H329.1C328.7 313.1 328.4 314.3 328 315.4V312zM640 368C640 447.5 575.5 512 496 512C416.5 512 352 447.5 352 368C352 288.5 416.5 224 496 224C575.5 224 640 288.5 640 368zM540.7 324.7L480 385.4L451.3 356.7C445.1 350.4 434.9 350.4 428.7 356.7C422.4 362.9 422.4 373.1 428.7 379.3L468.7 419.3C474.9 425.6 485.1 425.6 491.3 419.3L563.3 347.3C569.6 341.1 569.6 330.9 563.3 324.7C557.1 318.4 546.9 318.4 540.7 324.7H540.7z" /></ svg > } }