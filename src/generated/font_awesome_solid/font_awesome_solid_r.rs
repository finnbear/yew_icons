use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_r (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M228.7 309.7C282 288.6 320 236.8 320 176c0-79.41-64.59-144-144-144H32c-17.67 0-32 14.33-32 32v384c0 17.67 14.33 32 32 32s32-14.33 32-32v-128h93.43l104.5 146.6c6.25 8.75 16.09 13.42 26.09 13.42c6.422 0 12.91-1.922 18.55-5.938c14.39-10.27 17.73-30.25 7.484-44.64L228.7 309.7zM64 96.01h112c44.11 0 80 35.89 80 80s-35.89 79.1-80 79.1H64V96.01z" /></ svg > } }