use crate :: IconProps ; # [inline (never)] pub fn simple_icons_clubhouse (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M23.184 2.654l-10.967 3.5V2.696L.39 6.47v10.025l10.2-3.258v3.441l13.41-4.275-3.634-3.55zM10.592 4.929v6.592l-8.567 2.733V7.662zm9.683.367l-1.85 3.9 2.542 2.467-8.75 2.791V7.871zM1.741 17.863c-.958 0-1.741.783-1.741 1.741 0 .959.783 1.742 1.741 1.742a1.74 1.74 0 100-3.483z" /></ svg > } }