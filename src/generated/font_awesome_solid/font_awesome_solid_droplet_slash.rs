use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_droplet_slash (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M215.3 143.4C243.5 95.07 274.2 49.29 294.9 19.3C307.2 1.585 332.8 1.585 345.1 19.3C393.7 89.43 496 245.9 496 319.1C496 333.7 494.4 347.1 491.5 359.9L630.8 469.1C641.2 477.3 643.1 492.4 634.9 502.8C626.7 513.2 611.6 515.1 601.2 506.9L9.196 42.89C-1.236 34.71-3.065 19.63 5.112 9.196C13.29-1.236 28.37-3.065 38.81 5.112L215.3 143.4zM143.1 319.1C143.1 296.5 154.3 264.6 169.1 229.9L443.5 445.4C411.7 476.7 368.1 496 319.1 496C222.8 496 143.1 417.2 143.1 319.1V319.1zM239.1 319.1C239.1 311.2 232.8 303.1 223.1 303.1C215.2 303.1 207.1 311.2 207.1 319.1C207.1 381.9 258.1 432 319.1 432C328.8 432 336 424.8 336 416C336 407.2 328.8 400 319.1 400C275.8 400 239.1 364.2 239.1 319.1V319.1z" /></ svg > } }