use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_tent_arrow_turn_left (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M86.54 72H456C522.3 72 576 125.7 576 192V232C576 245.3 565.3 256 552 256C538.7 256 528 245.3 528 232V192C528 152.2 495.8 120 456 120H86.54L120.1 150.2C129.9 159 130.7 174.2 121.8 184.1C112.1 193.9 97.8 194.7 87.94 185.8L7.945 113.8C2.888 109.3 0 102.8 0 96C0 89.2 2.888 82.71 7.945 78.16L87.94 6.161C97.8-2.706 112.1-1.907 121.8 7.945C130.7 17.8 129.9 32.97 120.1 41.84L86.54 72zM475.4 294.5C482 299.6 486.4 307 487.6 315.3L511.6 475.3C513 484.5 510.3 493.8 504.2 500.9C498.2 507.9 489.3 512 480 512H384L287.1 352V512H96C86.68 512 77.83 507.9 71.75 500.9C65.67 493.8 62.97 484.5 64.35 475.3L88.35 315.3C89.59 307 93.98 299.6 100.6 294.5L268.6 166.5C280.1 157.8 295.9 157.8 307.4 166.5L475.4 294.5z" /></ svg > } }