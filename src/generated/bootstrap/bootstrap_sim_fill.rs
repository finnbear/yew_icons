use crate :: IconProps ; # [inline (never)] pub fn bootstrap_sim_fill (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/twbs/icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M5 4.5a.5.5 0 0 1 .5-.5h2v2H5V4.5zM8.5 6V4h2a.5.5 0 0 1 .5.5V6H8.5zM5 7h6v2H5V7zm3.5 3H11v1.5a.5.5 0 0 1-.5.5h-2v-2zm-1 0v2h-2a.5.5 0 0 1-.5-.5V10h2.5z" /> < path d = "M3.5 0A1.5 1.5 0 0 0 2 1.5v13A1.5 1.5 0 0 0 3.5 16h9a1.5 1.5 0 0 0 1.5-1.5V3.414a1.5 1.5 0 0 0-.44-1.06L11.647.439A1.5 1.5 0 0 0 10.586 0H3.5zm2 3h5A1.5 1.5 0 0 1 12 4.5v7a1.5 1.5 0 0 1-1.5 1.5h-5A1.5 1.5 0 0 1 4 11.5v-7A1.5 1.5 0 0 1 5.5 3z" /> </ svg > } }