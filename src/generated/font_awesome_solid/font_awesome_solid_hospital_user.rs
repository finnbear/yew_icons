use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_hospital_user (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M272 0C298.5 0 320 21.49 320 48V367.8C281.8 389.2 256 430 256 476.9C256 489.8 259.6 501.8 265.9 512H48C21.49 512 0 490.5 0 464V384H144C152.8 384 160 376.8 160 368C160 359.2 152.8 352 144 352H0V288H144C152.8 288 160 280.8 160 272C160 263.2 152.8 256 144 256H0V48C0 21.49 21.49 0 48 0H272zM152 64C143.2 64 136 71.16 136 80V104H112C103.2 104 96 111.2 96 120V136C96 144.8 103.2 152 112 152H136V176C136 184.8 143.2 192 152 192H168C176.8 192 184 184.8 184 176V152H208C216.8 152 224 144.8 224 136V120C224 111.2 216.8 104 208 104H184V80C184 71.16 176.8 64 168 64H152zM512 272C512 316.2 476.2 352 432 352C387.8 352 352 316.2 352 272C352 227.8 387.8 192 432 192C476.2 192 512 227.8 512 272zM288 477.1C288 425.7 329.7 384 381.1 384H482.9C534.3 384 576 425.7 576 477.1C576 496.4 560.4 512 541.1 512H322.9C303.6 512 288 496.4 288 477.1V477.1z" /></ svg > } }