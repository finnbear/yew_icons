use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_people_line (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M360 72C360 94.09 342.1 112 320 112C297.9 112 280 94.09 280 72C280 49.91 297.9 32 320 32C342.1 32 360 49.91 360 72zM104 168C104 145.9 121.9 128 144 128C166.1 128 184 145.9 184 168C184 190.1 166.1 208 144 208C121.9 208 104 190.1 104 168zM608 416C625.7 416 640 430.3 640 448C640 465.7 625.7 480 608 480H32C14.33 480 0 465.7 0 448C0 430.3 14.33 416 32 416H608zM456 168C456 145.9 473.9 128 496 128C518.1 128 536 145.9 536 168C536 190.1 518.1 208 496 208C473.9 208 456 190.1 456 168zM200 352C200 369.7 185.7 384 168 384H120C102.3 384 88 369.7 88 352V313.5L61.13 363.4C54.85 375 40.29 379.4 28.62 373.1C16.95 366.8 12.58 352.3 18.87 340.6L56.75 270.3C72.09 241.8 101.9 224 134.2 224H153.8C170.1 224 185.7 228.5 199.2 236.6L232.7 174.3C248.1 145.8 277.9 128 310.2 128H329.8C362.1 128 391.9 145.8 407.3 174.3L440.8 236.6C454.3 228.5 469.9 224 486.2 224H505.8C538.1 224 567.9 241.8 583.3 270.3L621.1 340.6C627.4 352.3 623 366.8 611.4 373.1C599.7 379.4 585.2 375 578.9 363.4L552 313.5V352C552 369.7 537.7 384 520 384H472C454.3 384 440 369.7 440 352V313.5L413.1 363.4C406.8 375 392.3 379.4 380.6 373.1C368.1 366.8 364.6 352.3 370.9 340.6L407.2 273.1C405.5 271.5 404 269.6 402.9 267.4L376 217.5V272C376 289.7 361.7 304 344 304H295.1C278.3 304 263.1 289.7 263.1 272V217.5L237.1 267.4C235.1 269.6 234.5 271.5 232.8 273.1L269.1 340.6C275.4 352.3 271 366.8 259.4 373.1C247.7 379.4 233.2 375 226.9 363.4L199.1 313.5L200 352z" /></ svg > } }