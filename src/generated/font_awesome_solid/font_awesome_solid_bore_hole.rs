use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_bore_hole (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M256 0C273.7 0 288 14.33 288 32V296.6C307.1 307.6 320 328.3 320 352C320 387.3 291.3 416 256 416C220.7 416 192 387.3 192 352C192 328.3 204.9 307.6 224 296.6V32C224 14.33 238.3 0 256 0zM160 128V352C160 405 202.1 448 256 448C309 448 352 405 352 352V128H464C490.5 128 512 149.5 512 176V464C512 490.5 490.5 512 464 512H48C21.49 512 0 490.5 0 464V176C0 149.5 21.49 128 48 128H160z" /></ svg > } }