use crate :: IconProps ; # [inline (never)] pub fn simple_icons_designernews (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11.27 11.98c0-3.83-2.354-6.43-6.84-6.43H0v12.9h4.524c4.354 0 6.747-2.624 6.747-6.464v-.005zM8.056 12c0 2.766-1.42 3.963-3.7 3.963h-1.16V8.037h1.16c2.185 0 3.7 1.252 3.7 3.963zM24 18.45V5.55h-2.97v7.213L16.28 5.55h-3.105v12.9h2.973v-7.723l5.084 7.718H24v.004z" /></ svg > } }