use crate :: IconProps ; # [inline (never)] pub fn octicons_ruby_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M5.873 3.26A.75.75 0 016.44 3h11.31a.75.75 0 01.576.27l5 6a.75.75 0 01-.028.992l-10.75 11.5a.75.75 0 01-1.096 0l-10.75-11.5a.75.75 0 01-.02-1.003l5.19-6zm.91 1.24L2.258 9.73 12 20.153l9.75-10.43L17.399 4.5H6.783z" /></ svg > } }