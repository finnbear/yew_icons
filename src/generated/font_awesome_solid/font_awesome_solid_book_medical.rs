use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_book_medical (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 448 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M448 336v-288C448 21.49 426.5 0 400 0H96C42.98 0 0 42.98 0 96v320c0 53.02 42.98 96 96 96h320c17.67 0 32-14.33 32-31.1c0-11.72-6.607-21.52-16-27.1v-81.36C441.8 362.8 448 350.2 448 336zM128 166c0-8.838 7.164-16 16-16h53.1V96c0-8.838 7.164-16 16-16h52c8.836 0 16 7.162 16 16v54H336c8.836 0 16 7.162 16 16v52c0 8.836-7.164 16-16 16h-54V288c0 8.836-7.164 16-16 16h-52c-8.836 0-16-7.164-16-16V234H144c-8.836 0-16-7.164-16-16V166zM384 448H96c-17.67 0-32-14.33-32-32c0-17.67 14.33-32 32-32h288V448z" /></ svg > } }