use crate :: IconProps ; # [inline (never)] pub fn simple_icons_pluralsight (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M15.72 1.755C10.08-.301 3.811 2.625 1.771 8.25c-2.071 5.699.854 11.956 6.494 14.01 5.655 2.055 11.956-.87 14.01-6.51 2.057-5.67-.87-11.939-6.524-13.995h-.031zM12 24C5.4 24 0 18.6 0 12S5.4 0 12 0s12 5.4 12 12-5.4 12-12 12M8.926 5.805v12.391L19.68 12 8.926 5.805zm1.049 1.769L17.625 12l-7.65 4.426V7.574M6.449 7.155v9.689L14.85 12 6.449 7.155zm1.051 1.8L12.811 12 7.5 15.061V8.939" /></ svg > } }