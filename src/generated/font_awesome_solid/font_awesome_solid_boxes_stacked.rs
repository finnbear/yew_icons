use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_boxes_stacked (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M160 48C160 21.49 181.5 0 208 0H256V80C256 88.84 263.2 96 272 96H304C312.8 96 320 88.84 320 80V0H368C394.5 0 416 21.49 416 48V176C416 202.5 394.5 224 368 224H208C181.5 224 160 202.5 160 176V48zM96 288V368C96 376.8 103.2 384 112 384H144C152.8 384 160 376.8 160 368V288H208C234.5 288 256 309.5 256 336V464C256 490.5 234.5 512 208 512H48C21.49 512 0 490.5 0 464V336C0 309.5 21.49 288 48 288H96zM416 288V368C416 376.8 423.2 384 432 384H464C472.8 384 480 376.8 480 368V288H528C554.5 288 576 309.5 576 336V464C576 490.5 554.5 512 528 512H368C341.5 512 320 490.5 320 464V336C320 309.5 341.5 288 368 288H416z" /></ svg > } }