use crate :: IconProps ; # [inline (never)] pub fn octicons_rows_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M22 3.75A1.75 1.75 0 0020.25 2H3.75A1.75 1.75 0 002 3.75v5.5c0 .966.784 1.75 1.75 1.75h16.5A1.75 1.75 0 0022 9.25v-5.5zm-1.75-.25a.25.25 0 01.25.25v5.5a.25.25 0 01-.25.25H3.75a.25.25 0 01-.25-.25v-5.5a.25.25 0 01.25-.25h16.5zM22 14.75A1.75 1.75 0 0020.25 13H3.75A1.75 1.75 0 002 14.75v5.5c0 .966.784 1.75 1.75 1.75h16.5A1.75 1.75 0 0022 20.25v-5.5zm-1.75-.25a.25.25 0 01.25.25v5.5a.25.25 0 01-.25.25H3.75a.25.25 0 01-.25-.25v-5.5a.25.25 0 01.25-.25h16.5z" /></ svg > } }