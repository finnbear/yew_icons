use crate :: IconProps ; # [inline (never)] pub fn simple_icons_momenteo (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M17.925 6.615c-.6.01-1.154.323-1.472.831-.348.41-3.163 3.98-4.142 5.22l.765.968c1.036-1.306 4.096-5.169 4.243-5.348a.765.765 0 011.265 0c.344.42 1.624 2.047 2.703 3.408.482.591.926 1.213 1.328 1.862.626 1.043-.395 2.02-.792 2.457l-3.254-4.098a.811.811 0 00-1.25-.016L14.2 15.836 7.548 7.447a1.774 1.774 0 00-3.02.024c-.059.067-1.706 2.156-2.989 3.776-.528.701-.956 1.33-1.178 1.7-1.048 1.75.441 3.462 1.239 4.165.174.16.399.257.636.272a.727.727 0 00.677-.368l3.145-3.97s2.882 3.644 3.227 4.07a.64.64 0 001.033-.005c.198-.253.76-.962 1.373-1.733l-.765-.964c-.548.69-1.021 1.286-1.127 1.426l-3.118-3.938a.811.811 0 00-1.25.016l-3.254 4.099c-.397-.438-1.416-1.415-.792-2.458a17.57 17.57 0 011.329-1.861c1.078-1.362 2.358-2.989 2.703-3.408a.765.765 0 011.264 0l7 8.823a.64.64 0 001.034.005c.345-.426 3.227-4.07 3.227-4.07l3.146 3.968a.727.727 0 00.675.367c.238-.015.463-.11.638-.272.797-.702 2.286-2.414 1.238-4.165-.222-.37-.65-1-1.179-1.7-1.282-1.621-2.929-3.71-2.989-3.777a1.774 1.774 0 00-1.546-.854z" /></ svg > } }