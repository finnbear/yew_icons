use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_bowling_ball (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM144 208c-17.7 0-32-14.25-32-32s14.3-32 32-32s32 14.25 32 32S161.7 208 144 208zM240 80c17.66 0 31.95 14.25 31.95 32s-14.29 32-31.95 32s-32.05-14.25-32.05-32S222.4 80 240 80zM240 240c-17.7 0-32-14.25-32-32s14.3-32 32-32s32 14.25 32 32S257.7 240 240 240z" /></ svg > } }