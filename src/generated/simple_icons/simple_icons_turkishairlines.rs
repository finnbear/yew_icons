use crate :: IconProps ; # [inline (never)] pub fn simple_icons_turkishairlines (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M.168 13.988c.272 1.623.86 3.115 1.69 4.423 3.095-.863 5.817-2.495 6.785-6.132 1.065-4.003-.15-8.199-3.057-10.422C1.626 4.364-.657 9.077.168 13.988m23.664-3.975c1.098 6.534-3.308 12.722-9.844 13.819-1.1.185-2.19.214-3.245.103a12.023 12.023 0 0 1-8.46-4.892l19.428-5.57c.279-.08.207-.349-.024-.333l-8.145.569c1.148-1.108 2.021-2.467 1.915-4.345-.214-3.043-3.311-6.013-9.071-7.967a12.016 12.016 0 0 1 6.87-1.333c5.228.548 9.663 4.512 10.576 9.95" /></ svg > } }