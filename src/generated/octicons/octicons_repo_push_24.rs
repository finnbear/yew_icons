use crate :: IconProps ; # [inline (never)] pub fn octicons_repo_push_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M4.75 0A2.75 2.75 0 002 2.75v16.5A2.75 2.75 0 004.75 22h11a.75.75 0 000-1.5h-11c-.69 0-1.25-.56-1.25-1.25V18A1.5 1.5 0 015 16.5h7.25a.75.75 0 000-1.5H5c-.546 0-1.059.146-1.5.401V2.75c0-.69.56-1.25 1.25-1.25H18.5v7a.75.75 0 001.5 0V.75a.75.75 0 00-.75-.75H4.75z" />< path d = "M20 13.903l2.202 2.359a.75.75 0 001.096-1.024l-3.5-3.75a.75.75 0 00-1.096 0l-3.5 3.75a.75.75 0 101.096 1.024l2.202-2.36v9.348a.75.75 0 001.5 0v-9.347z" /></ svg > } }