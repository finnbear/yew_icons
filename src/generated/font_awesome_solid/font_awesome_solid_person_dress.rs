use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_person_dress (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M112 48C112 21.49 133.5 0 160 0C186.5 0 208 21.49 208 48C208 74.51 186.5 96 160 96C133.5 96 112 74.51 112 48zM88 384H70.2C59.28 384 51.57 373.3 55.02 362.9L93.28 248.1L59.43 304.5C50.33 319.6 30.67 324.5 15.52 315.4C.3696 306.3-4.531 286.7 4.573 271.5L58.18 182.3C78.43 148.6 114.9 128 154.2 128H165.8C205.1 128 241.6 148.6 261.8 182.3L315.4 271.5C324.5 286.7 319.6 306.3 304.5 315.4C289.3 324.5 269.7 319.6 260.6 304.5L226.7 248.1L264.1 362.9C268.4 373.3 260.7 384 249.8 384H232V480C232 497.7 217.7 512 200 512C182.3 512 168 497.7 168 480V384H152V480C152 497.7 137.7 512 120 512C102.3 512 88 497.7 88 480L88 384z" /></ svg > } }