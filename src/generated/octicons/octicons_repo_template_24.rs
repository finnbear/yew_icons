use crate :: IconProps ; # [inline (never)] pub fn octicons_repo_template_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M5.75 0A2.75 2.75 0 003 2.75v1a.75.75 0 001.5 0v-1c0-.69.56-1.25 1.25-1.25h1a.75.75 0 000-1.5h-1zm4 0a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zm7.5 0a.75.75 0 000 1.5h2.25v2.25a.75.75 0 001.5 0v-3a.75.75 0 00-.75-.75h-3zM4.5 6.5a.75.75 0 00-1.5 0v3.75a.75.75 0 001.5 0V6.5zm16.5 0a.75.75 0 00-1.5 0v3.75a.75.75 0 001.5 0V6.5zM4.5 13.25a.75.75 0 00-1.5 0v5.5a3.25 3.25 0 001.95 2.98.75.75 0 10.6-1.375A1.75 1.75 0 014.5 18.75V18A1.5 1.5 0 016 16.5h.75a.75.75 0 000-1.5H6c-.546 0-1.059.146-1.5.401V13.25zm16.5 0a.75.75 0 00-1.5 0V15h-2.25a.75.75 0 000 1.5h2.25v4h-5.25a.75.75 0 000 1.5h6a.75.75 0 00.75-.75v-8zM9.75 15a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5zm-2.353 8.461A.25.25 0 017 23.26v-5.01a.25.25 0 01.25-.25h5a.25.25 0 01.25.25v5.01a.25.25 0 01-.397.201l-2.206-1.604a.25.25 0 00-.294 0L7.397 23.46z" /></ svg > } }