use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_stairs (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M576 64c0 17.67-14.31 32-32 32h-96v96c0 17.67-14.31 32-32 32h-96v96c0 17.67-14.31 32-32 32H192v96c0 17.67-14.31 32-32 32H32c-17.69 0-32-14.33-32-32s14.31-32 32-32h96v-96c0-17.67 14.31-32 32-32h96V192c0-17.67 14.31-32 32-32h96V64c0-17.67 14.31-32 32-32h128C561.7 32 576 46.33 576 64z" /></ svg > } }