use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_house_medical_flag (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M480 0C497.7 0 512 14.33 512 32H624C632.8 32 640 39.16 640 48V176C640 184.8 632.8 192 624 192H512V512H448V32C448 14.33 462.3 0 480 0V0zM416 512H416.1L416.8 512H96C78.33 512 64 497.7 64 480V288H31.1C18.61 288 6.631 279.7 1.985 267.1C-2.661 254.5 1.005 240.4 11.17 231.7L235.2 39.7C247.2 29.43 264.8 29.43 276.8 39.7L416 159V512zM223.1 256H175.1C167.2 256 159.1 263.2 159.1 272V304C159.1 312.8 167.2 320 175.1 320H223.1V368C223.1 376.8 231.2 384 239.1 384H271.1C280.8 384 287.1 376.8 287.1 368V320H336C344.8 320 352 312.8 352 304V272C352 263.2 344.8 256 336 256H287.1V208C287.1 199.2 280.8 192 271.1 192H239.1C231.2 192 223.1 199.2 223.1 208V256z" /></ svg > } }