use crate :: IconProps ; # [inline (never)] pub fn font_awesome_solid_school_lock (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 640 512" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M336 160H352C360.8 160 368 167.2 368 176C368 184.8 360.8 192 352 192H320C311.2 192 304 184.8 304 176V144C304 135.2 311.2 128 320 128C328.8 128 336 135.2 336 144V160zM302.2 5.374C312.1-1.791 327-1.791 337.8 5.374L476.8 98.06L602.4 125.1C624.4 130.9 640 150.3 640 172.8V271.1C640 210.1 589.9 159.1 528 159.1C466.1 159.1 416 210.1 416 271.1V296.6C396.9 307.6 384 328.3 384 352H320.3L320 352C284.7 352 256 380.7 256 416V512H320L48 512C21.49 512 0 490.5 0 464V172.8C0 150.3 15.63 130.9 37.59 125.1L163.2 98.06L302.2 5.374zM80 272C80 280.8 87.16 288 96 288H128C136.8 288 144 280.8 144 272V208C144 199.2 136.8 192 128 192H96C87.16 192 80 199.2 80 208V272zM80 400C80 408.8 87.16 416 96 416H128C136.8 416 144 408.8 144 400V336C144 327.2 136.8 320 128 320H96C87.16 320 80 327.2 80 336V400zM320 264C368.6 264 408 224.6 408 176C408 127.4 368.6 88 320 88C271.4 88 232 127.4 232 176C232 224.6 271.4 264 320 264zM528 192C572.2 192 608 227.8 608 272V320C625.7 320 640 334.3 640 352V480C640 497.7 625.7 512 608 512H448C430.3 512 416 497.7 416 480V352C416 334.3 430.3 320 448 320V272C448 227.8 483.8 192 528 192zM528 240C510.3 240 496 254.3 496 272V320H560V272C560 254.3 545.7 240 528 240z" /></ svg > } }