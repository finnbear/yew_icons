use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_magnet (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 448 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M128 160V256C128 309 170.1 352 224 352C277 352 320 309 320 256V160H448V256C448 379.7 347.7 480 224 480C100.3 480 0 379.7 0 256V160H128zM0 64C0 46.33 14.33 32 32 32H96C113.7 32 128 46.33 128 64V128H0V64zM320 64C320 46.33 334.3 32 352 32H416C433.7 32 448 46.33 448 64V128H320V64z" /></ svg > } }