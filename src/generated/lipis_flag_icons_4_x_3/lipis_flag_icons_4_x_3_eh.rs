use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_eh (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-eh" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs id = "defs13030" > < clippath id = "eh-a" > < path id = "path13027" fill - opacity = ".7" d = "M-158.7 0H524v512h-682.7z" /> </ clippath > </ defs > < g id = "g13044" fill - rule = "evenodd" transform = "translate(148.8) scale(.94)" > < path id = "rect13190" fill = "#000" d = "M-158.3 0h680.9v255.3h-680.9z" style = "stroke-width:1.38978" /> < path id = "rect13192" fill = "#007a3d" d = "M-158.3 255.3h680.9v255.3h-680.9z" style = "stroke-width:1.38978" /> < path id = "rect13194" fill = "#fff" d = "M-158.3 148.9h680.9v212.8h-680.9z" style = "stroke-width:1.55382" /> < path id = "path13196" fill = "#c4111b" d = "m-158.3 0 340.4 255.3-340.4 255.3Z" style = "stroke-width:1.70213" /> < circle id = "circle13198" cx = "352.3" cy = "255.3" r = "68.1" fill = "#c4111b" style = "stroke-width:1.70213" /> < circle id = "circle13200" cx = "377.9" cy = "255.3" r = "68.1" fill = "#fff" style = "stroke-width:1.70213" /> < path id = "path13202" fill = "#c4111b" d = "m334 296.5 29.1-20.7 28.8 21-10.8-34 29-20.9-35.7-.2-11-34-11.2 33.9-35.7-.2 28.7 21.2-11.1 34z" style = "stroke-width:1.70213" /> </ g > </ svg > } }