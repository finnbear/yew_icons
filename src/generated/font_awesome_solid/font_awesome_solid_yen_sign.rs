use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_yen_sign (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M159.1 198.3L261.4 46.25C271.2 31.54 291 27.57 305.8 37.37C320.5 47.18 324.4 67.04 314.6 81.75L219.8 223.1H272C289.7 223.1 304 238.3 304 255.1C304 273.7 289.7 287.1 272 287.1H192V319.1H272C289.7 319.1 304 334.3 304 352C304 369.7 289.7 384 272 384H192V448C192 465.7 177.7 480 159.1 480C142.3 480 127.1 465.7 127.1 448V384H47.1C30.33 384 15.1 369.7 15.1 352C15.1 334.3 30.33 319.1 47.1 319.1H127.1V287.1H47.1C30.33 287.1 15.1 273.7 15.1 255.1C15.1 238.3 30.33 223.1 47.1 223.1H100.2L5.374 81.75C-4.429 67.04-.456 47.18 14.25 37.37C28.95 27.57 48.82 31.54 58.62 46.25L159.1 198.3z" /></ svg > } }