use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_0 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M160 32.01c-88.37 0-160 71.63-160 160v127.1c0 88.37 71.63 160 160 160s160-71.63 160-160V192C320 103.6 248.4 32.01 160 32.01zM256 320c0 52.93-43.06 96-96 96c-52.93 0-96-43.07-96-96V192c0-52.94 43.07-96 96-96c52.94 0 96 43.06 96 96V320z" /></ svg > } }