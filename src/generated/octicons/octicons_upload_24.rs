use crate :: IconProps ; # [inline (never)] pub fn octicons_upload_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M4.75 22a.75.75 0 010-1.5h14.5a.75.75 0 010 1.5H4.75zm.22-13.53a.75.75 0 001.06 1.06L11 4.56v12.19a.75.75 0 001.5 0V4.56l4.97 4.97a.75.75 0 101.06-1.06l-6.25-6.25a.75.75 0 00-1.06 0L4.97 8.47z" /></ svg > } }