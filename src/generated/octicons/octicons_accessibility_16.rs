use crate :: IconProps ; # [inline (never)] pub fn octicons_accessibility_16 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M9.923 5.302a3 3 0 10-3.847 0A2.713 2.713 0 005.9 5.5H2A.75.75 0 002 7h3.3l-.578 5.163-.362 2.997a.75.75 0 101.49.18L6.132 13h3.736l.282 2.34a.75.75 0 101.49-.18l-.362-2.997L10.7 7H14a.75.75 0 000-1.5h-3.899a2.697 2.697 0 00-.178-.198zM9.5 3a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zm-.3 4.073l.495 4.427h-3.39l.496-4.427a1.207 1.207 0 012.398 0z" /></ svg > } }