use crate :: IconProps ; # [inline (never)] pub fn simple_icons_loop (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12,0C5.371,0,0,5.371,0,12s5.371,12,12,12s12-5.371,12-12C24.011,5.371,18.629,0,12,0z M12.7,22.611 C6.837,22.611,2.089,17.863,2.089,12S6.837,1.389,12.7,1.389S23.311,6.137,23.311,12S18.563,22.611,12.7,22.611z M7.045,3.413 c-4.747,2.735-6.366,8.795-3.632,13.542c2.735,4.737,8.806,6.366,13.542,3.632c4.747-2.735,6.366-8.806,3.632-13.542 C17.852,2.297,11.792,0.678,7.045,3.413z M16.868,19.034c-4.08,2.352-9.287,0.952-11.639-3.118 c-2.352-4.08-0.952-9.287,3.118-11.639c4.08-2.352,9.287-0.952,11.639,3.118C22.337,11.464,20.948,16.682,16.868,19.034z  M5.229,8.084c-2.166,3.741-0.875,8.532,2.866,10.687c3.741,2.166,8.532,0.875,10.698-2.866s0.875-8.532-2.866-10.687 C12.175,3.063,7.384,4.343,5.229,8.084z M18.071,14.702c-1.827,3.161-5.863,4.244-9.025,2.417 c-3.161-1.827-4.244-5.863-2.418-9.025s5.863-4.244,9.025-2.418C18.815,7.493,19.898,11.541,18.071,14.702z M6.093,12 c0,3.271,2.647,5.918,5.918,5.918s5.918-2.647,5.918-5.918s-2.647-5.918-5.918-5.918C8.74,6.082,6.093,8.729,6.093,12z  M16.704,11.3c0,2.593-2.1,4.693-4.693,4.693s-4.693-2.1-4.693-4.693s2.1-4.693,4.693-4.693C14.593,6.607,16.704,8.707,16.704,11.3 z" /></ svg > } }