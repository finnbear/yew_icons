use crate :: IconProps ; # [inline (never)] pub fn simple_icons_lunacy (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12.031 6h-6v12h11.996v-6l-5.996 5.996Zm6.563 2.309a4.013 4.013 0 0 1-2.371-2.375 4.03 4.03 0 0 1-2.375 2.375 4.04 4.04 0 0 1 2.375 2.375 4.013 4.013 0 0 1 2.37-2.375ZM0 9.602c0-3.364 0-5.043.652-6.325A6.044 6.044 0 0 1 3.277.652C4.56 0 6.238 0 9.602 0h4.796c3.364 0 5.043 0 6.325.652a6.044 6.044 0 0 1 2.625 2.625C24 4.56 24 6.238 24 9.602v4.796c0 3.364 0 5.043-.652 6.325a6.044 6.044 0 0 1-2.625 2.625C19.44 24 17.762 24 14.398 24H9.602c-3.364 0-5.043 0-6.325-.652a6.044 6.044 0 0 1-2.625-2.625C0 19.44 0 17.762 0 14.398Z" /></ svg > } }