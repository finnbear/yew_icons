use crate :: IconProps ; # [inline (never)] pub fn simple_icons_backbonedotjs (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M2.34 0v10.45l3.2-1.83V5.27l2.93 1.67 3.01-1.72L2.34 0zm19.31 0L12.5 5.22l3.02 1.73 2.94-1.68v3.35l3.2 1.83V0h-.01zm-9.9 5.64-9.4 5.38V24l9.4-5.36v-3.76l-6.21 3.56v-5.5l6.21-3.54V5.64zm.5 0V9.4l6.22 3.54v5.5l-6.22-3.56v3.76L21.66 24V11.02l-9.41-5.38zM7.7 12.3l-1.65.94v1.86l2.17 1.24 3.28-1.87-3.8-2.17zm8.61 0-3.8 2.16 3.28 1.88 2.17-1.24v-1.86l-1.65-.94z" /></ svg > } }