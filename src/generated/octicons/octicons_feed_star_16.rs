use crate :: IconProps ; # [inline (never)] pub fn octicons_feed_star_16 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M8 16A8 8 0 108 0a8 8 0 000 16zm.252-12.932a.478.478 0 00-.682.195l-1.2 2.432-2.684.39a.478.478 0 00-.266.816l1.944 1.892-.46 2.674a.478.478 0 00.694.504L8 10.709l2.4 1.261a.478.478 0 00.694-.504l-.458-2.673L12.578 6.9a.479.479 0 00-.265-.815l-2.685-.39-1.2-2.432a.478.478 0 00-.176-.195z" /></ svg > } }