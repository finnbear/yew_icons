use crate :: IconProps ; # [inline (never)] pub fn simple_icons_modin (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M6.4403 7.3914H8.506a.3583.3583 0 0 0 .3536-.3536.3583.3583 0 0 0-.3536-.3583H6.4403a.3583.3583 0 1 0 0 .7119zm14.0794 7.0816a1.7402 1.7402 0 0 0-1.7727 1.7774 1.7727 1.7727 0 0 0 1.7727 1.796h1.703A1.7727 1.7727 0 0 0 24 16.2735a1.7448 1.7448 0 0 0-1.7773-1.7774zm1.703-4.9506A1.7448 1.7448 0 0 0 24 7.745a1.7727 1.7727 0 0 0-1.7773-1.7727h-1.703A1.7727 1.7727 0 0 0 18.747 7.745a1.7402 1.7402 0 0 0 1.7727 1.7774zm-1.703.6979a1.7774 1.7774 0 0 0-1.7727 1.7774 1.7402 1.7402 0 0 0 1.7727 1.7774h1.703A1.7448 1.7448 0 0 0 24 11.9977a1.7774 1.7774 0 0 0-1.7773-1.7728zm-4.8854 4.2527a1.7402 1.7402 0 0 0-1.7728 1.7774 1.768 1.768 0 0 0 1.7728 1.796h.6327a1.7727 1.7727 0 0 0 1.7774-1.7728 1.7448 1.7448 0 0 0-1.7774-1.8006zm.6327-4.9506a1.7448 1.7448 0 0 0 1.7774-1.7774 1.7727 1.7727 0 0 0-1.7774-1.7727h-.6514A1.768 1.768 0 0 0 13.843 7.745a1.7402 1.7402 0 0 0 1.7727 1.7774zm-.6514.6979a1.7774 1.7774 0 0 0-1.754 1.7774 1.7402 1.7402 0 0 0 1.7727 1.7774h.6327a1.7448 1.7448 0 0 0 1.796-1.7774 1.7774 1.7774 0 0 0-1.796-1.7774zM3.9138 8.8244h1.033a.3536.3536 0 0 0 .3582-.3536.363.363 0 0 0-.3583-.3582H3.9324a.3536.3536 0 0 0 0 .7072zm-3.15-2.131h.2699a.3536.3536 0 0 0 .3583-.3536.363.363 0 0 0-.3583-.3582H.7638a.3536.3536 0 0 0 0 .7072zm5.6765 9.9106H8.506a.3536.3536 0 0 1 .3536.3536.3583.3583 0 0 1-.3536.3583H6.4403a.3583.3583 0 1 1 0-.712zm-2.5079-1.4191h1.033a.3536.3536 0 1 1 0 .7072h-1.033a.3536.3536 0 0 1 0-.7072zm-3.15 2.131h.2699a.3536.3536 0 0 1 .3583.3536.363.363 0 0 1-.3583.3582H.7824a.3536.3536 0 0 1 0-.7072zm1.5867.7304h9.0264a1.7727 1.7727 0 0 0 1.7728-1.7727 1.7402 1.7402 0 0 0-1.7728-1.7774H1.6944a.3583.3583 0 0 0 0 .712h.9306a.3536.3536 0 0 1 0 .7072H.3172a.3583.3583 0 0 0 0 .7118H5.17a.3583.3583 0 0 1 0 .712H2.3784a.3536.3536 0 0 0-.3397.3535.3583.3583 0 0 0 .3536.3583Zm4.0712-6.3883H8.506a.3536.3536 0 0 0 .3536-.3536.3583.3583 0 0 0-.3536-.3583H6.4403a.363.363 0 0 0-.3583.3583.3583.3583 0 0 0 .3583.3536zm-2.5079 1.4238h1.033a.3583.3583 0 1 0 0-.7119h-1.033a.3536.3536 0 0 0-.3536.3536.3583.3583 0 0 0 .3536.3583zm-3.15-2.131h.2699a.3536.3536 0 1 0 0-.7072H.7824a.3583.3583 0 0 0-.3536.3536.3583.3583 0 0 0 .3536.3582zm1.5867-.7119h9.0264a1.7774 1.7774 0 0 1 1.7728 1.7588 1.7402 1.7402 0 0 1-1.7728 1.7774H1.6944a.3536.3536 0 0 1 0-.7073h.9306a.3583.3583 0 0 0 .3536-.3582.3583.3583 0 0 0-.3536-.3537H.3172a.3583.3583 0 0 1 0-.7118H5.17a.3583.3583 0 0 0 0-.712H2.3784a.3583.3583 0 0 1 0-.7118zm0-4.2527h9.0264a1.7727 1.7727 0 0 1 1.7728 1.7727 1.7402 1.7402 0 0 1-1.7728 1.7774H1.6944a.3583.3583 0 0 1-.3536-.3536.3536.3536 0 0 1 .3396-.3583h.9306a.3536.3536 0 0 0 0-.7072H.3172a.3583.3583 0 0 1 0-.7119H5.17a.3583.3583 0 0 0 0-.7118H2.3784a.3536.3536 0 0 1-.3397-.3537.3583.3583 0 0 1 .3304-.3908Z" /></ svg > } }