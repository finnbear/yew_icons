use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_truck_moving (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M416 32C451.3 32 480 60.65 480 96V144H528.8C545.6 144 561.5 151.5 572.2 164.5L630.1 236.4C636.8 243.5 640 252.5 640 261.7V352C640 369.7 625.7 384 608 384H606.4C607.4 389.2 608 394.5 608 400C608 444.2 572.2 480 528 480C483.8 480 448 444.2 448 400C448 394.5 448.6 389.2 449.6 384H286.4C287.4 389.2 288 394.5 288 400C288 444.2 252.2 480 208 480C181.8 480 158.6 467.4 144 448C129.4 467.4 106.2 480 80 480C35.82 480 0 444.2 0 400V96C0 60.65 28.65 32 64 32H416zM535 194.9C533.5 193.1 531.2 192 528.8 192H480V256H584.1L535 194.9zM528 432C545.7 432 560 417.7 560 400C560 382.3 545.7 368 528 368C510.3 368 496 382.3 496 400C496 417.7 510.3 432 528 432zM208 368C190.3 368 176 382.3 176 400C176 417.7 190.3 432 208 432C225.7 432 240 417.7 240 400C240 382.3 225.7 368 208 368zM80 432C97.67 432 112 417.7 112 400C112 382.3 97.67 368 80 368C62.33 368 48 382.3 48 400C48 417.7 62.33 432 80 432z" /></ svg > } }