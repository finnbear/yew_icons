use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_radiation (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M256 303.1c26.5 0 48-21.5 48-48S282.5 207.1 256 207.1S208 229.5 208 255.1S229.5 303.1 256 303.1zM213.6 188L142.7 74.71C132.5 58.41 109.9 54.31 95.25 66.75c-44.94 38.1-76.19 91.82-85.17 152.8C7.266 238.7 22.67 255.8 42.01 255.8h133.8C175.8 227.2 191 202.3 213.6 188zM416.8 66.75c-14.67-12.44-37.21-8.338-47.41 7.965L298.4 188c22.6 14.3 37.8 39.2 37.8 67.8h133.8c19.34 0 34.74-17.13 31.93-36.26C492.9 158.6 461.7 104.8 416.8 66.75zM298.4 323.5C286.1 331.2 271.6 335.9 256 335.9s-30.1-4.701-42.4-12.4L142.7 436.9c-10.14 16.21-4.16 38.2 13.32 45.95C186.6 496.4 220.4 504 256 504s69.42-7.611 100-21.18c17.48-7.752 23.46-29.74 13.32-45.95L298.4 323.5z" /></ svg > } }