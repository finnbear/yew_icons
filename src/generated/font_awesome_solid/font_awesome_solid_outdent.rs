use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_outdent (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M32 64C32 46.33 46.33 32 64 32H448C465.7 32 480 46.33 480 64C480 81.67 465.7 96 448 96H64C46.33 96 32 81.67 32 64V64zM224 192C224 174.3 238.3 160 256 160H448C465.7 160 480 174.3 480 192C480 209.7 465.7 224 448 224H256C238.3 224 224 209.7 224 192zM448 288C465.7 288 480 302.3 480 320C480 337.7 465.7 352 448 352H256C238.3 352 224 337.7 224 320C224 302.3 238.3 288 256 288H448zM32 448C32 430.3 46.33 416 64 416H448C465.7 416 480 430.3 480 448C480 465.7 465.7 480 448 480H64C46.33 480 32 465.7 32 448V448zM32.24 268.6C24 262.2 24 249.8 32.24 243.4L134.2 164.1C144.7 155.9 160 163.4 160 176.7V335.3C160 348.6 144.7 356.1 134.2 347.9L32.24 268.6z" /></ svg > } }