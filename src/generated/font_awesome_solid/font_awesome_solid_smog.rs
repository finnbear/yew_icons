use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_smog (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M144 288h156.1C322.6 307.8 351.8 320 384 320s61.25-12.25 83.88-32H528C589.9 288 640 237.9 640 176s-50.13-112-112-112c-18 0-34.75 4.625-49.75 12.12C453.1 30.1 406.8 0 352 0c-41 0-77.75 17.25-104 44.75C221.8 17.25 185 0 144 0c-79.5 0-144 64.5-144 144S64.5 288 144 288zM136 464H23.1C10.8 464 0 474.8 0 487.1S10.8 512 23.1 512H136C149.2 512 160 501.2 160 488S149.2 464 136 464zM616 368h-528C74.8 368 64 378.8 64 391.1S74.8 416 87.1 416h528c13.2 0 24-10.8 24-23.1S629.2 368 616 368zM552 464H231.1C218.8 464 208 474.8 208 487.1S218.8 512 231.1 512H552c13.2 0 24-10.8 24-23.1S565.2 464 552 464z" /></ svg > } }