use crate :: IconProps ; # [inline (never)] pub fn octicons_cloud_16 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M2 7.25A5.225 5.225 0 017.25 2a5.222 5.222 0 014.767 3.029A4.472 4.472 0 0116 9.5c0 2.505-1.995 4.5-4.5 4.5h-8A3.475 3.475 0 010 10.5c0-1.41.809-2.614 2.001-3.17L2 7.25zm1.54.482a.75.75 0 01-.556.832c-.86.22-1.484.987-1.484 1.936 0 1.124.876 2 2 2h8c1.676 0 3-1.324 3-3s-1.324-3-3-3a.75.75 0 01-.709-.504A3.72 3.72 0 007.25 3.5C5.16 3.5 3.5 5.16 3.5 7.25a3.276 3.276 0 00.035.436l.004.036.001.008v.002z" /></ svg > } }