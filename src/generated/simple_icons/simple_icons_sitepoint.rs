use crate :: IconProps ; # [inline (never)] pub fn simple_icons_sitepoint (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M2.471 10.533l1.771 1.688 5.598 5.141 2.4-2.291c.21-.297.194-.705-.046-.985L9.99 12.184l.01-.005-2.371-2.266c-.279-.314-.27-.78.021-1.079l6.39-6.076L11.146 0 2.475 8.238c-.664.633-.664 1.66 0 2.295h-.004zm19.056 2.937l-1.77-1.691-5.595-5.142-2.411 2.291c-.221.3-.207.705.045.985l2.205 1.891h-.006l2.369 2.265c.27.314.27.766-.029 1.064l-6.391 6.075L12.855 24l8.67-8.238c.664-.633.666-1.659 0-2.295l.002.003z" /></ svg > } }