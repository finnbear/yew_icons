use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_pump_medical (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 384 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M379.3 94.06l-43.32-43.32C323.1 38.74 307.7 32 290.8 32h-66.75c0-17.67-14.33-32-32-32H127.1c-17.67 0-32 14.33-32 32L96 128h128l-.0002-32h66.75l43.31 43.31c6.248 6.248 16.38 6.248 22.63 0l22.62-22.62C385.6 110.4 385.6 100.3 379.3 94.06zM235.6 160H84.37C51.27 160 23.63 185.2 20.63 218.2l-20.36 224C-3.139 479.7 26.37 512 64.01 512h191.1c37.63 0 67.14-32.31 63.74-69.79l-20.36-224C296.4 185.2 268.7 160 235.6 160zM239.1 333.3c0 7.363-5.971 13.33-13.33 13.33h-40v40c0 7.363-5.969 13.33-13.33 13.33h-26.67c-7.363 0-13.33-5.971-13.33-13.33v-40H93.33c-7.363 0-13.33-5.971-13.33-13.33V306.7c0-7.365 5.971-13.33 13.33-13.33h40v-40C133.3 245.1 139.3 240 146.7 240h26.67c7.363 0 13.33 5.969 13.33 13.33v40h40c7.363 0 13.33 5.969 13.33 13.33V333.3z" /></ svg > } }