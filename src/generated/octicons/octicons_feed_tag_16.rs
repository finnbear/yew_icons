use crate :: IconProps ; # [inline (never)] pub fn octicons_feed_tag_16 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M7.22 6.5a.72.72 0 11-1.44 0 .72.72 0 011.44 0z" />< path fill - rule = "evenodd" d = "M8 16A8 8 0 108 0a8 8 0 000 16zM4 8.379V5a1 1 0 011-1h3.379a1.5 1.5 0 011.06.44l3.213 3.211a1.2 1.2 0 010 1.698l-3.303 3.303a1.2 1.2 0 01-1.698 0L4.44 9.439A1.5 1.5 0 014 8.38z" /></ svg > } }