use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_diagram_next (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M512 160C512 195.3 483.3 224 448 224H280V288H326.1C347.4 288 358.1 313.9 343 328.1L272.1 399C263.6 408.4 248.4 408.4 239 399L168.1 328.1C153.9 313.9 164.6 288 185.9 288H232V224H64C28.65 224 0 195.3 0 160V96C0 60.65 28.65 32 64 32H448C483.3 32 512 60.65 512 96V160zM312.6 416H448V352H376.6L384.1 343.6C401 327.6 404.6 306.4 399 288H448C483.3 288 512 316.7 512 352V416C512 451.3 483.3 480 448 480H64C28.65 480 0 451.3 0 416V352C0 316.7 28.65 288 64 288H112.1C107.4 306.4 110.1 327.6 127 343.6L135.4 352H64V416H199.4L216.4 432.1C238.3 454.8 273.7 454.8 295.6 432.1L312.6 416z" /></ svg > } }