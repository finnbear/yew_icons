use crate :: IconProps ; # [inline (never)] pub fn simple_icons_citrix (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11.983 0a1.78 1.78 0 0 0-1.78 1.78 1.78 1.78 0 0 0 1.78 1.78 1.78 1.78 0 0 0 1.78-1.78A1.78 1.78 0 0 0 11.983 0zM5.17 5.991a1.026 1.026 0 0 0-1.095 1.027c0 .308.136.616.376.822l6.162 7.086-6.401 7.258a1.084 1.084 0 0 0-.309.787c0 .582.48 1.027 1.062 1.027.342 0 .684-.17.89-.444l6.128-7.19 6.162 7.19c.205.274.547.444.89.444.582.035 1.062-.445 1.062-1.027a1.14 1.14 0 0 0-.309-.787l-6.402-7.258 6.162-7.086c.24-.206.377-.514.377-.822v-.034c0-.582-.513-1.027-1.095-.993-.343 0-.65.171-.856.445l-5.957 7.018L6.06 6.436a1.07 1.07 0 0 0-.855-.445z" /></ svg > } }