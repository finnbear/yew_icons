use crate :: IconProps ; # [inline (never)] pub fn simple_icons_windowsxp (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M9.302 1.415c-1.184.02-2.592.32-4.341 1.044l-2.283 7.949c1.846-.761 3.311-1.114 4.537-1.1a7.596 7.596 0 014.37 1.593l2.296-7.92c-1.26-.855-2.607-1.599-4.58-1.566zm5.75 2.411l-2.256 7.949c2.016 1.367 4.44 2.494 8.907.493L24 4.333h-.042c-4.651 1.931-6.906.846-8.907-.507zM6.617 10.77c-1.184.018-2.591.315-4.335 1.034L0 19.779c4.65-1.93 6.863-.803 8.878.55l2.326-7.99c-1.26-.855-2.613-1.6-4.586-1.57zm5.784 2.344l.011.008.003-.008zm.011.008l-2.294 7.898c2.015 1.367 4.256 2.453 8.906.522l2.297-7.92c-4.641 1.927-6.882.85-8.909-.5Z" /></ svg > } }