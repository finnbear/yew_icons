use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_cart_flatbed (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M240 320h320c26.4 0 48-21.6 48-48v-192C608 53.6 586.4 32 560 32H448v128l-48-32L352 160V32H240C213.6 32 192 53.6 192 80v192C192 298.4 213.6 320 240 320zM608 384H128V64c0-35.2-28.8-64-64-64H31.1C14.4 0 0 14.4 0 32S14.4 64 31.1 64H48C56.84 64 64 71.16 64 80v335.1c0 17.6 14.4 32 32 32l66.92-.0009C161.1 453 160 458.4 160 464C160 490.5 181.5 512 208 512S256 490.5 256 464c0-5.641-1.13-10.97-2.917-16h197.9c-1.787 5.027-2.928 10.36-2.928 16C448 490.5 469.5 512 496 512c26.51 0 48.01-21.49 48.01-47.1c0-5.641-1.12-10.97-2.907-16l66.88 .0009C625.6 448 640 433.6 640 415.1C640 398.4 625.6 384 608 384z" /></ svg > } }