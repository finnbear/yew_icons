use crate :: IconProps ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_vu (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } id = "flag-icons-vu" viewBox = "0 0 640 480" > if let Some (title) = title . clone () { < title > { title } </ title > } < defs > < clippath id = "vu-a" > < path fill - opacity = ".7" d = "M0 0h682.7v512H0z" /> </ clippath > </ defs > < g transform = "scale(.9375)" > < g fill - rule = "evenodd" > < path d = "m0 0 347.4 219.4H768v73.2H347.4L0 512V0z" /> < path fill = "#ff0" d = "M0 493.7 354.7 267H768v-22H354.7L0 18.3v32.9L332.8 256 0 460.8v32.9z" /> < path fill = "#40aa40" d = "m0 512 354.7-226.7H768V512H0z" /> < path fill = "#ce0000" d = "m0 0 354.7 226.7H768V0H0z" /> < path fill = "#ff0" d = "M95.8 266.6c1.2.3 1.6.6 2.8-1.1.2-1 1-2.2 1.7-3.3.8-1.5 1.2-2 2-1 .6.7 2.8-.5 3.6.1 1.4 1 .6.8 1.7-.2.9-1.8.1-1.6-1-2.4-1-.6-3.1.6-4 0 .3-1.8.9-2 2-2.4.8.5 3.1-.4 3.8-.4 1 .2 2 .3 2.3-1.2.5-1 .3-.3-.2-1-.8-.7-3 .5-3.9 0-1-.9-.9-1.2-.2-2.6.8-.5 1.5-.3 2.6.4 1 .8 2.8-.7 3.8-.8.9-1 1.9-1.3 1.2-2.5-.3-1-.6-1-1.8-1.3-1.2-.8-2.8.7-3.3-.5 1-1.4 1.3-1.1 2.5-.3.8.2 3-1.1 3.8-1.2 1.1-.3.6 1 1.4-1.8-.3-1.2-2.3.7-3.4.4-1 .2-2-.2-2-1.4 0-1.5.8-1.6 1.9-1.4 1 .2 3.1-1 4-1 1 .6 1.5.4 2.4-1 .6-1.7-.1-1-1.3-2-.8-.5-3 .7-3.9.1.2-.6.5-1.4 1-1.7.9.2 1.6.2 2.7.8 1 .5 2.8-1.5 3.7-2.2-.2-1-2 .4-3-.3-.6-.5-1.5-1-1.7-1.5.6-1.7.3-1.6 2-1 .8-.3 2.3-.3 1.3-1.9-.2-.2-1-.2-1.8-.5-.9-.6-1.7-1.3-2.7-1.5-.7-.1-1.7-.4-2.2-.2 0 1 .2 1.6.1 2.8.5.7 1.3.7 1.5 1-.9.8-1.4.4-2.3.4-1.2-.7-.5-3.1-1.9-2.6.3.7.2 3.4.8 4 .6.4 1.4.8 1.5 1.3-1 1.5-1.3 1.4-2.5.7-1-.6-.6-2.8-1.5-2.7-1 .8-1 .7-.8 1.8 0 1.4-.5 3.4.7 4.2 1.3 1 1.5.8.4 2.7-.7.9-1.1.7-2 .3-.9-.6-.7-3-1.4-3.7-1.4-.8-.6-.8-1.7.2-.3 1.3 0 1.6.5 2.7.4.7.4 3 1.2 3.2 1.2.7 1.3.6-.2 2-1 0-1.6.3-2.4-.7-1-.8-.4-3.2-1.7-3.2-1.2.1-1.5.1-1.4 1.8.3 1.5-.3 3.8.9 4.7 1.1.5 1.8.4 2.2 1.1-.4.3-1 1.3-1.4 1.5-.8 0-1.6-.5-2.5-.7-1-.5-.5-1.4-1.4-2-1 .3-1-1-1.5.3.2 1.2-.2 2.6 1 3.4.8.5 1.5 1.7 2.4 2.3 1 1.2.6 1.4 0 3-.9.1-1.8-.5-2.7-1-.9-.7-.7-3-1.6-3.5-.7-.8-.5-1.4-1.7.2 0 1 .1 1.2.4 2 0 1-.2 3.2.8 3.9 1 .2 2.2.7 3 1 .8 1 .1.8-.8 2.2-.5 1.4-.5 1.7-1.4 2.4-.8 1-1 1.5-.4 2.8z" /> < path fill = "#ff0" d = "M121 267.9c.7-1.2 1.1-1.4 0-3.4-.8-.7-1.6-2-2.4-3.1-1-1.6-1.3-2.2 0-2.7.8-.3.6-3 1.5-3.7 1.4-1.1 1-.3.5-1.9-1.4-1.7-1.5-.8-2.7.2-1 .7-.7 3.5-1.5 4.2-1.6-1-1.6-1.8-1.5-3 .9-.7.9-3.6 1.2-4.3.6-1 1-2-.2-2.9-.8-1-.2-.4-1-.3-1 .7-.7 3.5-1.6 4.2-1.2.8-1.4.5-2.4-.8-.1-1 .3-1.7 1.4-2.6 1.1-.8.5-3.3.8-4.3-.6-1.4-.5-2.6-1.8-2.4-1 0-1.2.3-2 1.5-1.2 1-.4 3.2-1.8 3.2-.9-1.6-.4-1.8.8-2.8.5-.7.2-3.7.4-4.5.2-1.2 1-.2-1.1-2-1.2-.3-.2 2.6-1 3.6-.3 1.2-1 2-2 1.6-1.5-.6-1.2-1.5-.6-2.5s.3-3.8.7-4.7c1-1 1-1.4 0-3-1.3-1.4-1-.3-2.2.6-1 .7-.7 3.5-1.6 4.2-.5-.5-1-1-1.1-1.8.5-.8.7-1.6 1.8-2.5.8-.9-.4-3.5-.6-4.8-1-.2-.4 2.2-1.5 3-.7.5-1.4 1.2-2 1.2-1.3-1.3-1.4-1-.2-2.5.1-1 .7-2.5-1.2-2-.3 0-.5 1-1.1 1.6-1 .7-1.9 1.3-2.5 2.2-.4.7-1 1.7-1 2.3.8.5 1.5.5 2.6 1 .8-.2 1.2-1 1.5-1.1.3 1.2-.2 1.6-.5 2.6-1.2 1-3.2-.7-3.2 1 .8-.1 3.2 1 4 .7.6-.5 1.3-1.2 1.8-1 1 1.5.8 1.9-.3 2.8-1 .8-3-.4-3.1.5.3 1.4.2 1.4 1.3 1.6 1.3.5 2.9 1.9 4 1 1.5-1 1.5-1.3 2.7.6.6 1.1.3 1.5-.5 2.2-.9.8-3-.4-4 0-1.2 1.2-.9.4-.4 2 1 .8 1.4.7 2.7.5.8-.2 3 .8 3.4 0 1.1-1 1-1.1 1.7 1-.4 1.2-.3 1.8-1.6 2.3-1.1.8-3-.9-3.6.5-.4 1.4-.5 1.7 1.1 2.2 1.5.3 3.4 1.8 4.7 1 .9-1 1-1.8 1.9-2 .1.6.7 1.6.7 2.1-.2.9-1 1.5-1.6 2.4-.8.9-1.5 0-2.3.7-.1 1.1-1.4.6-.3 1.7 1 .2 2.3 1.2 3.4.4.9-.7 2.3-1 3.1-1.7 1.5-.6 1.5 0 2.8 1.2-.3 1-1.2 1.7-2 2.4-1 .7-3-.4-3.9.3-1 .5-1.4 0-.5 1.9 1 .5 1.2.3 2 .3 1 .5 2.8 1.6 3.9.8.6-1 1.6-2 2.2-2.8 1.1-.4.7.2 1.6 1.7 1.1 1.2 1.4 1.3 1.7 2.5.6 1.2 1 1.7 2.4 1.5z" /> < path fill = "#ff0" d = "M98 314.5c0-5.3-.8-10.6 0-10.6 46.8 0 58.5-32 58.5-53.2S140.3 208 109.7 208c-35 0-46.8 20.8-46.8 42.6A37 37 0 0 0 98 287.9c23.4 0 29.3-5.3 46.8-26.6-5.8 26.6-35 37.2-46.8 37.2-23.4 0-46.8-16-46.8-47.8 0-26.6 17.5-53.2 58.5-53.2 35.1 0 58.5 26.6 58.5 53.2 0 37.2-29.2 63.8-70.2 63.8z" /> </ g > </ g > </ svg > } }