use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_bridge_circle_xmark (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M576 32C593.7 32 608 46.33 608 64C608 81.67 593.7 96 576 96H536V160H608V232.2C577.6 207.1 538.5 192 496 192C426.9 192 367.1 231.8 338.3 289.7C332.4 288.6 326.3 288 320 288C266.1 288 224 330.1 224 384V448C224 465.7 209.7 480 192 480H160C142.3 480 128 465.7 128 448V384C128 330.1 85.02 288 32 288V160H104V96H64C46.33 96 32 81.67 32 64C32 46.33 46.33 32 64 32H576zM488 96H408V160H488V96zM280 96V160H360V96H280zM232 96H152V160H232V96zM352 368C352 288.5 416.5 224 496 224C575.5 224 640 288.5 640 368C640 447.5 575.5 512 496 512C416.5 512 352 447.5 352 368zM555.3 331.3C561.6 325.1 561.6 314.9 555.3 308.7C549.1 302.4 538.9 302.4 532.7 308.7L496 345.4L459.3 308.7C453.1 302.4 442.9 302.4 436.7 308.7C430.4 314.9 430.4 325.1 436.7 331.3L473.4 368L436.7 404.7C430.4 410.9 430.4 421.1 436.7 427.3C442.9 433.6 453.1 433.6 459.3 427.3L496 390.6L532.7 427.3C538.9 433.6 549.1 433.6 555.3 427.3C561.6 421.1 561.6 410.9 555.3 404.7L518.6 368L555.3 331.3z" /></ svg > } }