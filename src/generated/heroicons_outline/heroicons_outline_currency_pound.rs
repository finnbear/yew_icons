use crate :: IconProps ; # [inline (never)] pub fn heroicons_outline_currency_pound (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/tailwindlabs/heroicons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "none" viewBox = "0 0 24 24" stroke - width = "1.5" stroke = "currentColor" aria - hidden = "true" > if let Some (title) = title . clone () { < title > { title } </ title > } < path stroke - linecap = "round" stroke - linejoin = "round" d = "M14.121 7.629A3 3 0 009.017 9.43c-.023.212-.002.425.028.636l.506 3.541a4.5 4.5 0 01-.43 2.65L9 16.5l1.539-.513a2.25 2.25 0 011.422 0l.655.218a2.25 2.25 0 001.718-.122L15 15.75M8.25 12H12m9 0a9 9 0 11-18 0 9 9 0 0118 0z" /> </ svg > } }