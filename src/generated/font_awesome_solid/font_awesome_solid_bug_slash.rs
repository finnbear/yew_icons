use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_bug_slash (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M239.1 162.8C247.7 160.1 255.7 160 264 160H376C393.1 160 409.2 164.1 423.5 171.4C424.1 170.7 424.7 170 425.4 169.4L489.4 105.4C501.9 92.88 522.1 92.88 534.6 105.4C547.1 117.9 547.1 138.1 534.6 150.6L470.6 214.6C469.1 215.3 469.3 215.9 468.6 216.5C474.7 228.5 478.6 241.9 479.7 256H544C561.7 256 576 270.3 576 288C576 305.7 561.7 320 544 320H480C480 329.9 479.1 339.5 477.4 348.9L630.8 469.1C641.2 477.3 643.1 492.4 634.9 502.8C626.7 513.2 611.6 515.1 601.2 506.9L9.196 42.89C-1.236 34.71-3.065 19.63 5.112 9.196C13.29-1.236 28.37-3.065 38.81 5.112L239.1 162.8zM416 96V99.56C416 115.3 403.3 128 387.6 128H252.4C236.7 128 224 115.3 224 99.56V96C224 42.98 266.1 .001 320 .001C373 .001 416 42.98 416 96V96zM160.3 256C161.1 245.1 163.3 236.3 166.7 227.3L304 335.5V479.2C269.5 475.8 238.2 461.4 213.7 439.6L150.6 502.6C138.1 515.1 117.9 515.1 105.4 502.6C92.88 490.1 92.88 469.9 105.4 457.4L169.4 393.4C171.2 391.5 173.3 389.9 175.4 388.6C165.5 367.8 160 344.6 160 320H96C78.33 320 64 305.7 64 288C64 270.3 78.33 256 96 256H160.3zM336 479.2V360.7L430.8 435.4C405.7 459.6 372.7 475.6 336 479.2V479.2z" /></ svg > } }