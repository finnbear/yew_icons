use crate :: IconProps ; # [inline (never)] pub fn octicons_dependabot_24 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 24 24" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M8.75 11a.75.75 0 01.75.75v3.5a.75.75 0 01-1.5 0v-3.5a.75.75 0 01.75-.75zm7.25.75a.75.75 0 00-1.5 0v3.5a.75.75 0 001.5 0v-3.5z" />< path fill - rule = "evenodd" d = "M9.813 1a.75.75 0 000 1.5H11.5V5H4.25A2.25 2.25 0 002 7.25v5.25H.75a.75.75 0 000 1.5H2v5.75A2.25 2.25 0 004.25 22h15.5A2.25 2.25 0 0022 19.75V14h1.25a.75.75 0 000-1.5H22V7.25A2.25 2.25 0 0019.75 5H13V1.75a.75.75 0 00-.75-.75H9.812zM3.5 7.25a.75.75 0 01.75-.75h15.5a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75H4.25a.75.75 0 01-.75-.75V7.25z" /></ svg > } }