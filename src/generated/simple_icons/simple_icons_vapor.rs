use crate :: IconProps ; # [inline (never)] pub fn simple_icons_vapor (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M22.75 13.908v1.56L12 24 1.25 15.468v-1.56L12 22.44l10.75-8.532zM12 17.267L1.25 8.824 12 0l10.75 8.824L12 17.267zm.356-4.635a3.193 3.193 0 0 0 3.193-3.193 3.185 3.185 0 0 0-3.029-3.176l.001-.016-4.514-.427 1.205 4.102a3.184 3.184 0 0 0 3.144 2.71zM12 20.269L1.25 11.737v1.533L12 21.802l10.75-8.532v-1.533L12 20.269zm0-2.366L1.25 9.46v1.64L12 19.63l10.75-8.532V9.46L12 17.903z" /></ svg > } }