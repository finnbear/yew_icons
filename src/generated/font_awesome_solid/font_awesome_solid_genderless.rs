use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_genderless (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 384 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M192 80C94.83 80 16 158.8 16 256c0 97.17 78.83 176 176 176s176-78.83 176-176C368 158.8 289.2 80 192 80zM192 352c-52.95 0-96-43.05-96-96c0-52.95 43.05-96 96-96s96 43.05 96 96C288 308.9 244.1 352 192 352z" /></ svg > } }