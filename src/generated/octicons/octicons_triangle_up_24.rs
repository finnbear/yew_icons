use crate :: IconProps ; # [inline (never)] pub fn octicons_triangle_up_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12.354 8.854l5.792 5.792a.5.5 0 01-.353.854H6.207a.5.5 0 01-.353-.854l5.792-5.792a.5.5 0 01.708 0z" /></ svg > } }