use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_circle_h (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM368 360c0 13.25-10.75 24-24 24S320 373.3 320 360v-80H192v80C192 373.3 181.3 384 168 384S144 373.3 144 360v-208C144 138.8 154.8 128 168 128S192 138.8 192 152v80h128v-80C320 138.8 330.8 128 344 128s24 10.75 24 24V360z" /></ svg > } }