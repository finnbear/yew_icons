use crate :: IconProps ; # [inline (never)] pub fn simple_icons_adp (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M15.08584 11.9999a3.13031 3.13031 0 0 1-3.12003 3.12002h-1.2v-1.37144h1.2a1.74859 1.74859 0 1 0 0-3.49717h-1.2V8.87987h1.2a3.13031 3.13031 0 0 1 3.12003 3.12002M8.43436 8.87987v2.53716H6.27434l-.78858 1.37144H9.8058v-3.9086Zm15.56584 1.9543a4.28575 4.28575 0 0 1-4.28575 4.28575v2.33145h-3.70289V15.6342a5.36233 5.36233 0 0 1-4.08003 1.81716H8.43436v-2.33145H5.69148l-1.37144 2.33145H0L6.34291 6.54842h5.6229a5.59548 5.59548 0 0 1 4.08004 1.81716V6.54842h3.70289a4.2789 4.2789 0 0 1 4.25146 4.28575m-12.03439 5.24576a4.09032 4.09032 0 0 0 3.7029-2.33145h1.74858v2.33145h.96v-2.33145h1.37145a2.91088 2.91088 0 0 0 2.9143-2.91431 2.94174 2.94174 0 0 0-2.94859-2.91431H17.383v3.49717h-1.37144a4.11432 4.11432 0 0 0-4.04575-3.49717H7.16577l-4.76575 8.16007h1.13144l1.37144-2.33145h4.9029v2.33145zm7.74864-7.20006h-1.37144v1.37144h1.37144a.57943.57943 0 0 1 .58286.58286.6.6 0 0 1-.58286.58286h-1.37144v1.37144h1.37144a1.9543 1.9543 0 0 0 1.9543-1.9543 1.97487 1.97487 0 0 0-1.9543-1.9543 M21.63447 16.76565a.54858.54858 0 1 1-.54858-.54858.54172.54172 0 0 1 .54858.54858m.13714 0a.68572.68572 0 1 0-.68572.68572.68572.68572 0 0 0 .68572-.68572 M21.12018 16.45707a.13714.13714 0 1 1 0 .2743h-.13715v-.2743zm.17143-.03428a.26057.26057 0 0 0-.17143-.06857h-.2743v.82286h.10287v-.30857h.13714l.17143.30857h.13714l-.20571-.34286c.03428 0 .06857-.03429.10286-.06857a.20572.20572 0 0 0 .03428-.13715.192.192 0 0 0-.03428-.20571" /></ svg > } }