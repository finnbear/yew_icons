use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_car_battery (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M80 96C80 78.33 94.33 64 112 64H176C193.7 64 208 78.33 208 96H304C304 78.33 318.3 64 336 64H400C417.7 64 432 78.33 432 96H448C483.3 96 512 124.7 512 160V384C512 419.3 483.3 448 448 448H64C28.65 448 0 419.3 0 384V160C0 124.7 28.65 96 64 96H80zM384 192C384 183.2 376.8 176 368 176C359.2 176 352 183.2 352 192V224H320C311.2 224 304 231.2 304 240C304 248.8 311.2 256 320 256H352V288C352 296.8 359.2 304 368 304C376.8 304 384 296.8 384 288V256H416C424.8 256 432 248.8 432 240C432 231.2 424.8 224 416 224H384V192zM96 256H192C200.8 256 208 248.8 208 240C208 231.2 200.8 224 192 224H96C87.16 224 80 231.2 80 240C80 248.8 87.16 256 96 256z" /></ svg > } }