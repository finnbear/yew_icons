use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_hand_middle_finger (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 448 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M448 288v96c0 70.69-57.31 128-128 128H184c-50.35 0-97.76-23.7-127.1-63.98l-14.43-19.23C35.37 420.5 32 410.4 32 400v-48.02c0-14.58 6.629-28.37 18.02-37.48L80 290.5V336C80 344.8 87.16 352 96 352s16-7.164 16-16v-96C112 213.5 133.5 192 160 192s48 21.48 48 48V40C208 17.91 225.9 0 248 0S288 17.91 288 40v189.5C296.6 216.6 311.3 208 328 208c23.48 0 42.94 16.87 47.11 39.14C382.4 242.7 390.8 240 400 240C426.5 240 448 261.5 448 288z" /></ svg > } }