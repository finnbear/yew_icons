use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_prescription_bottle_medical (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 384 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M32 448c0 35.2 28.8 64 64 64h192c35.2 0 64-28.8 64-64V128H32V448zM96 304C96 295.2 103.2 288 112 288H160V240C160 231.2 167.2 224 176 224h32C216.8 224 224 231.2 224 240V288h48C280.8 288 288 295.2 288 304v32c0 8.799-7.199 16-16 16H224v48c0 8.799-7.199 16-16 16h-32C167.2 416 160 408.8 160 400V352H112C103.2 352 96 344.8 96 336V304zM360 0H24C10.75 0 0 10.75 0 24v48C0 85.25 10.75 96 24 96h336C373.3 96 384 85.25 384 72v-48C384 10.75 373.3 0 360 0z" /></ svg > } }