use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_bitcoin_sign (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 320 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M48 32C48 14.33 62.33 0 80 0C97.67 0 112 14.33 112 32V64H144V32C144 14.33 158.3 0 176 0C193.7 0 208 14.33 208 32V64C208 65.54 207.9 67.06 207.7 68.54C254.1 82.21 288 125.1 288 176C288 200.2 280.3 222.6 267.3 240.9C298.9 260.7 320 295.9 320 336C320 397.9 269.9 448 208 448V480C208 497.7 193.7 512 176 512C158.3 512 144 497.7 144 480V448H112V480C112 497.7 97.67 512 80 512C62.33 512 48 497.7 48 480V448H41.74C18.69 448 0 429.3 0 406.3V101.6C0 80.82 16.82 64 37.57 64H48V32zM176 224C202.5 224 224 202.5 224 176C224 149.5 202.5 128 176 128H64V224H176zM64 288V384H208C234.5 384 256 362.5 256 336C256 309.5 234.5 288 208 288H64z" /></ svg > } }