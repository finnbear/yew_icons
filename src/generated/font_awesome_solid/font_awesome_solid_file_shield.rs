use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_file_shield (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 576 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M0 64C0 28.65 28.65 0 64 0H224V128C224 145.7 238.3 160 256 160H384V207L291.2 244.2C269.9 252.7 256 273.3 256 296.2C256 352.7 274.9 444.2 350.2 504.4C341.2 509.3 330.9 512 320 512H64C28.65 512 0 483.3 0 448V64zM256 128V0L384 128H256zM423.1 225.7C428.8 223.4 435.2 223.4 440.9 225.7L560.9 273.7C570 277.4 576 286.2 576 296C576 359.3 550.1 464.8 441.2 510.2C435.3 512.6 428.7 512.6 422.8 510.2C313.9 464.8 288 359.3 288 296C288 286.2 293.1 277.4 303.1 273.7L423.1 225.7zM432 273.8V461.7C500.2 428.7 523.5 362.7 527.4 311.1L432 273.8z" /></ svg > } }