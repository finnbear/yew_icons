use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_glass_water (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 384 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M352 0C360.9 0 369.4 3.692 375.4 10.19C381.5 16.69 384.6 25.42 383.9 34.28L355.1 437.7C352.1 479.6 317.3 512 275.3 512H108.7C66.72 512 31.89 479.6 28.9 437.7L.0813 34.28C-.5517 25.42 2.527 16.69 8.58 10.19C14.63 3.692 23.12 0 32 0L352 0zM97.19 168.6C116.6 178.3 139.4 178.3 158.8 168.6C179.7 158.1 204.3 158.1 225.2 168.6C244.6 178.3 267.4 178.3 286.8 168.6L311 156.5L317.6 64H66.37L72.97 156.5L97.19 168.6z" /></ svg > } }