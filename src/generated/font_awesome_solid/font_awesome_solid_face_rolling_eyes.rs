use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_face_rolling_eyes (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM192 368C183.2 368 176 375.2 176 384C176 392.8 183.2 400 192 400H320C328.8 400 336 392.8 336 384C336 375.2 328.8 368 320 368H192zM186.2 165.6C189.8 170.8 192 177.1 192 184C192 201.7 177.7 216 160 216C142.3 216 128 201.7 128 184C128 177.1 130.2 170.8 133.8 165.6C111.5 175.6 96 197.1 96 224C96 259.3 124.7 288 160 288C195.3 288 224 259.3 224 224C224 197.1 208.5 175.6 186.2 165.6zM352 288C387.3 288 416 259.3 416 224C416 197.1 400.5 175.6 378.2 165.6C381.8 170.8 384 177.1 384 184C384 201.7 369.7 216 352 216C334.3 216 320 201.7 320 184C320 177.1 322.2 170.8 325.8 165.6C303.5 175.6 288 197.1 288 224C288 259.3 316.7 288 352 288z" /></ svg > } }