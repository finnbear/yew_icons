use crate :: IconProps ; # [inline (never)] pub fn octicons_zap_16 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M10.561 1.5a.016.016 0 00-.01.004L3.286 8.571A.25.25 0 003.462 9H6.75a.75.75 0 01.694 1.034l-1.713 4.188 6.982-6.793A.25.25 0 0012.538 7H9.25a.75.75 0 01-.683-1.06l2.008-4.418.003-.006a.02.02 0 00-.004-.009.02.02 0 00-.006-.006L10.56 1.5zM9.504.43a1.516 1.516 0 012.437 1.713L10.415 5.5h2.123c1.57 0 2.346 1.909 1.22 3.004l-7.34 7.142a1.25 1.25 0 01-.871.354h-.302a1.25 1.25 0 01-1.157-1.723L5.633 10.5H3.462c-1.57 0-2.346-1.909-1.22-3.004L9.503.429z" /></ svg > } }