use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_hands_clapping (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 512 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M320 96c8.844 0 16-7.156 16-16v-64C336 7.156 328.8 0 320 0s-16 7.156-16 16v64C304 88.84 311.2 96 320 96zM383.4 96c5.125 0 10.16-2.453 13.25-7.016l32.56-48c1.854-2.746 2.744-5.865 2.744-8.951c0-8.947-7.273-16.04-15.97-16.04c-5.125 0-10.17 2.465-13.27 7.02l-32.56 48C368.3 73.76 367.4 76.88 367.4 79.97C367.4 88.88 374.7 96 383.4 96zM384 357.5l0-163.9c0-6.016-4.672-33.69-32-33.69c-17.69 0-32.07 14.33-32.07 31.1L320 268.1L169.2 117.3C164.5 112.6 158.3 110.3 152.2 110.3c-13.71 0-24 11.21-24 24c0 6.141 2.344 12.28 7.031 16.97l89.3 89.3C227.4 243.4 228.9 247.2 228.9 251c0 3.8-1.45 7.6-4.349 10.5c-2.899 2.899-6.7 4.349-10.5 4.349c-3.8 0-7.6-1.45-10.5-4.349l-107.6-107.6C91.22 149.2 85.08 146.9 78.94 146.9c-13.71 0-24 11.21-24 24c0 6.141 2.344 12.28 7.031 16.97l107.6 107.6C172.5 298.4 173.9 302.2 173.9 305.1c0 3.8-1.45 7.6-4.349 10.5c-2.899 2.9-6.7 4.349-10.5 4.349c-3.8 0-7.6-1.45-10.5-4.349L59.28 227.2C54.59 222.5 48.45 220.1 42.31 220.1c-13.71 0-24 11.21-24 24c0 6.141 2.344 12.28 7.031 16.97l89.3 89.3c2.9 2.899 4.349 6.7 4.349 10.5c0 3.8-1.45 7.6-4.349 10.5c-2.899 2.899-6.7 4.349-10.5 4.349c-3.8 0-7.6-1.45-10.5-4.349L40.97 318.7C36.28 314 30.14 311.7 24 311.7c-13.71 0-23.99 11.26-23.99 24.05c0 6.141 2.332 12.23 7.02 16.92C112.6 458.2 151.3 512 232.3 512C318.1 512 384 440.9 384 357.5zM243.3 88.98C246.4 93.55 251.4 96 256.6 96c8.762 0 15.99-7.117 15.99-16.03c0-3.088-.8906-6.205-2.744-8.951l-32.56-48C234.2 18.46 229.1 15.98 223.1 15.98c-8.664 0-15.98 7.074-15.98 16.05c0 3.086 .8906 6.205 2.744 8.951L243.3 88.98zM480 160c-17.69 0-32 14.33-32 32v76.14l-32-32v121.4c0 94.01-63.31 141.5-78.32 152.2C345.1 510.9 352.6 512 360.3 512C446.1 512 512 440.9 512 357.5l-.0625-165.6C511.9 174.3 497.7 160 480 160z" /></ svg > } }