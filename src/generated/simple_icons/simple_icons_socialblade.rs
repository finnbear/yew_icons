use crate :: IconProps ; # [inline (never)] pub fn simple_icons_socialblade (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M2.323 16.688H0v1.893h2.323v-1.893ZM5.935 13.591H3.613v4.99h2.322v-4.99ZM9.548 14.796H7.226v3.785h2.322v-3.785ZM13.161 13.935H10.84v4.646h2.322v-4.646ZM16.774 12.043h-2.322v6.538h2.322v-6.538ZM20.387 10.065h-2.323v8.516h2.323v-8.516ZM24 5.42h-2.323v13.16H24V5.42Z" /></ svg > } }