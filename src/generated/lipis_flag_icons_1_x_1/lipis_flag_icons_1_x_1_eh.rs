use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_eh (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-eh" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs id = "defs13889" > < clippath id = "eh-a" > < path id = "path13886" fill - opacity = ".7" d = "M0 0h512v512H0z" /> </ clippath > </ defs > < path id = "rect13190" fill = "#000" d = "M0 0h512v256H0z" style = "stroke-width:1.20679" /> < path id = "rect13192" fill = "#007a3d" d = "M0 256h512v256H0z" style = "stroke-width:1.20679" /> < path id = "rect13194" fill = "#fff" d = "M0 149.3h512v213.3H0z" style = "stroke-width:1.34923" /> < path id = "path13196" fill = "#c4111b" d = "m0 0 256 256L0 512Z" style = "stroke-width:1.47802" /> < g id = "g13957" transform = "translate(-135 -6.5) scale(1.02539)" > < circle id = "circle13198" cx = "512" cy = "256" r = "68.3" fill = "#c4111b" style = "stroke-width:1.70667" /> < circle id = "circle13200" cx = "537.6" cy = "256" r = "68.3" fill = "#fff" style = "stroke-width:1.70667" /> < path id = "path13202" fill = "#c4111b" d = "m493.7 297.3 29-20.8 29 21.2-10.8-34.2 29-21-35.8-.2-11-34-11.3 33.9-35.7-.1 28.7 21.2-11.1 34z" style = "stroke-width:1.70667" /> </ g > </ svg > } }