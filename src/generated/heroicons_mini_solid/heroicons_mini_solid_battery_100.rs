use crate :: IconProps ; # [inline (never)] pub fn heroicons_mini_solid_battery_100 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/tailwindlabs/heroicons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } viewBox = "0 0 20 20" fill = "currentColor" aria - hidden = "true" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M4.75 8a.75.75 0 00-.75.75v2.5c0 .414.336.75.75.75h9.5a.75.75 0 00.75-.75v-2.5a.75.75 0 00-.75-.75h-9.5z" /> < path fill - rule = "evenodd" d = "M1 7.25A2.25 2.25 0 013.25 5h12.5A2.25 2.25 0 0118 7.25v1.085a1.5 1.5 0 011 1.415v.5a1.5 1.5 0 01-1 1.415v1.085A2.25 2.25 0 0115.75 15H3.25A2.25 2.25 0 011 12.75v-5.5zm2.25-.75a.75.75 0 00-.75.75v5.5c0 .414.336.75.75.75h12.5a.75.75 0 00.75-.75v-5.5a.75.75 0 00-.75-.75H3.25z" clip - rule = "evenodd" /> </ svg > } }