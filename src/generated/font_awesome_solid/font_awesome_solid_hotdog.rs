use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_hotdog (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M488.6 23.44c-31.06-31.19-81.76-31.16-112.8 .0313L24.46 374.8c-20.83 19.96-29.19 49.66-21.83 77.6c7.36 27.94 29.07 49.65 57.02 57.01c27.94 7.36 57.64-1 77.6-21.83l351.3-351.3C519.7 105.2 519.8 54.5 488.6 23.44zM438.8 118.4c-19.59 19.59-37.39 22.52-51.74 25.01c-12.97 2.246-22.33 3.867-34.68 16.22c-12.35 12.35-13.97 21.71-16.22 34.69c-2.495 14.35-5.491 32.19-25.08 51.78c-19.59 19.59-37.43 22.58-51.78 25.08C246.3 273.4 236.9 275.1 224.6 287.4c-12.35 12.35-13.97 21.71-16.22 34.68C205.9 336.4 202.9 354.3 183.3 373.9c-19.59 19.59-37.43 22.58-51.78 25.08C118.5 401.2 109.2 402.8 96.83 415.2c-6.238 6.238-16.34 6.238-22.58 0c-6.238-6.238-6.238-16.35 0-22.58c19.59-19.59 37.43-22.58 51.78-25.07c12.97-2.245 22.33-3.869 34.68-16.22c12.35-12.35 13.97-21.71 16.22-34.69c2.495-14.35 5.492-32.19 25.08-51.78s37.43-22.58 51.78-25.08c12.97-2.246 22.33-3.869 34.68-16.22s13.97-21.71 16.22-34.68c2.495-14.35 5.492-32.19 25.08-51.78c19.59-19.59 37.43-22.58 51.78-25.07c12.97-2.246 22.28-3.815 34.63-16.17c6.238-6.238 16.36-6.238 22.59 0C444.1 102.1 444.1 112.2 438.8 118.4zM32.44 321.5l290-290l-11.48-11.6c-24.95-24.95-63.75-26.57-86.58-3.743L17.1 223.4C-5.73 246.3-4.108 285.1 20.84 310L32.44 321.5zM480.6 189.5l-290 290l11.48 11.6c24.95 24.95 63.75 26.57 86.58 3.743l207.3-207.3c22.83-22.83 21.21-61.63-3.743-86.58L480.6 189.5z" /></ svg > } }