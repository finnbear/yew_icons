use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_venus_double (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M368 176c0-97.2-78.8-176-176-176c-97.2 0-176 78.8-176 176c0 86.26 62.1 157.9 144 172.1v35.05H112c-8.836 0-16 7.162-16 16v32c0 8.836 7.164 16 16 16H160v48c0 8.836 7.164 16 16 16h32c8.838 0 16-7.164 16-16v-48h48c8.838 0 16-7.164 16-16v-32c0-8.838-7.162-16-16-16H224v-35.05C305.9 333.9 368 262.3 368 176zM192 272c-52.93 0-96-43.07-96-96c0-52.94 43.07-96 96-96c52.94 0 96 43.06 96 96C288 228.9 244.9 272 192 272zM624 176C624 78.8 545.2 0 448 0c-39.02 0-74.95 12.85-104.1 34.34c18.38 19.7 32.94 42.91 42.62 68.58C403.2 88.83 424.5 80 448 80c52.94 0 96 43.06 96 96c0 52.93-43.06 96-96 96c-23.57 0-44.91-8.869-61.63-23.02c-9.572 25.45-23.95 48.54-42.23 68.23C365.1 332.7 389.3 344 416 348.1V384h-48c-8.836 0-16 7.162-16 16v32c0 8.836 7.164 16 16 16H416v48c0 8.836 7.164 16 16 16h32c8.838 0 16-7.164 16-16V448h48c8.838 0 16-7.164 16-16v-32c0-8.838-7.162-16-16-16H480v-35.05C561.9 333.9 624 262.3 624 176z" /></ svg > } }