use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_face_grin_tongue (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M256 0C397.4 0 512 114.6 512 256C512 368.9 438.9 464.7 337.5 498.8C346.7 484 352 466.6 352 448V401.1C376.3 383.5 395.6 359.5 407.4 331.5C412.4 319.7 400.4 309 388.1 312.8C348.4 324.9 303.7 331.8 256.3 331.8C208.9 331.8 164.1 324.9 124.5 312.8C112.2 309 100.2 319.7 105.2 331.5C116.9 359.3 135.1 383.1 160 400.7V448C160 466.6 165.3 484 174.5 498.8C73.07 464.7 0 368.9 0 256C0 114.6 114.6 .0003 256 .0003L256 0zM176.4 240C194 240 208.4 225.7 208.4 208C208.4 190.3 194 176 176.4 176C158.7 176 144.4 190.3 144.4 208C144.4 225.7 158.7 240 176.4 240zM336.4 176C318.7 176 304.4 190.3 304.4 208C304.4 225.7 318.7 240 336.4 240C354 240 368.4 225.7 368.4 208C368.4 190.3 354 176 336.4 176zM256 512C220.7 512 192 483.3 192 448V402.6C192 387.9 203.9 376 218.6 376H220.6C231.9 376 241.7 383.9 244.2 394.9C247 407.5 264.1 407.5 267.8 394.9C270.3 383.9 280.1 376 291.4 376H293.4C308.1 376 320 387.9 320 402.6V448C320 483.3 291.3 512 256 512V512z" /></ svg > } }