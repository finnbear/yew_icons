use crate :: IconProps ; # [inline (never)] pub fn font_awesome_regular_lemon (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 448 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M439.9 144.6c15.34-26.38 8.372-62.41-16.96-87.62c-25.21-25.32-61.22-32.26-87.61-16.95c-9.044 5.218-27.15 3.702-48.08 1.968c-50.78-4.327-127.4-10.73-207.6 69.56C-.6501 191.9 5.801 268.5 10.07 319.3c1.749 20.96 3.28 39.07-1.984 48.08c-15.35 26.4-8.357 62.45 16.92 87.57c16.26 16.37 37.05 25.09 56.83 25.09c10.89 0 21.46-2.64 30.83-8.092c9.013-5.249 27.12-3.718 48.08-1.968c50.69 4.233 127.4 10.7 207.6-69.56c80.27-80.28 73.82-156.9 69.56-207.7C436.2 171.8 434.7 153.7 439.9 144.6zM398.4 120.5c-12.87 22.09-10.67 48.41-8.326 76.25c4.155 49.3 8.841 105.2-55.67 169.7c-64.53 64.49-120.5 59.78-169.7 55.68c-27.85-2.328-54.12-4.53-76.26 8.311c-6.139 3.64-19.17 1.031-29.58-9.451c-10.39-10.33-12.95-23.35-9.372-29.49c12.87-22.09 10.67-48.41 8.326-76.25C53.72 265.1 49.04 210.1 113.5 145.5c48.27-48.27 91.71-57.8 131.2-57.8c13.28 0 26.12 1.078 38.52 2.125c27.9 2.359 54.17 4.561 76.26-8.311c6.123-3.577 19.18-1.031 29.49 9.357C399.4 101.2 402 114.4 398.4 120.5zM239.5 124.1c2.156 8.561-3.062 17.25-11.62 19.43C183.6 154.7 122.7 215.6 111.6 259.9C109.7 267.1 103.2 271.1 96.05 271.1c-1.281 0-2.593-.1562-3.905-.4687C83.58 269.3 78.4 260.6 80.52 252.1C94.67 195.8 163.8 126.7 220.1 112.5C228.8 110.4 237.3 115.5 239.5 124.1z" /></ svg > } }