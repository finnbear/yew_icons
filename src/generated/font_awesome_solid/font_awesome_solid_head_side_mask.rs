use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_head_side_mask (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M.1465 184.4C-2.166 244.2 23.01 298 63.99 334.9L63.99 512h160L224 316.5L3.674 156.2C1.871 165.4 .5195 174.8 .1465 184.4zM336 368H496L512 320h-255.1l.0178 192h145.9c27.55 0 52-17.63 60.71-43.76L464 464h-127.1c-8.836 0-16-7.164-16-16c0-8.838 7.164-16 16-16h138.7l10.67-32h-149.3c-8.836 0-16-7.164-16-16C320 375.2 327.2 368 336 368zM509.2 275c-20.1-47.13-48.49-151.8-73.11-186.8C397.6 33.63 334.5 0 266.1 0H200C117.1 0 42.48 50.57 13.25 123.7L239.2 288h272.6C511.8 283.7 511.1 279.3 509.2 275zM352 224c-17.62 0-32-14.38-32-32s14.38-32 32-32c17.62 0 31.1 14.38 31.1 32S369.6 224 352 224z" /></ svg > } }