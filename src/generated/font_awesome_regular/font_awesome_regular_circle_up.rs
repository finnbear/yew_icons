use crate :: IconProps ; # [inline (never)] pub fn font_awesome_regular_circle_up (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M272.9 135.7C268.3 130.8 261.9 128 255.2 128C247.5 128.3 241.1 130.9 237.5 135.8l-87.25 96C143.8 238.9 142.2 249 146.1 257.7C149.9 266.4 158.5 272 167.1 272h56L224 360c0 13.25 10.75 24 24 24h16c13.25 0 23.1-10.75 23.1-24L287.1 272h56c9.531 0 18.16-5.656 22-14.38c3.811-8.75 2.092-18.91-4.377-25.91L272.9 135.7zM256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208S370.7 464 256 464z" /></ svg > } }