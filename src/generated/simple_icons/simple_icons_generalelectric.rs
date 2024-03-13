use crate :: IconProps ; # [inline (never)] pub fn simple_icons_generalelectric (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12.001 24C5.385 24 0 18.647 0 11.999 0 5.385 5.385 0 12.001 0 18.614 0 24 5.385 24 11.999 24 18.647 18.614 24 12.001 24zm0-23.436C5.684.564.565 5.685.565 12c0 6.316 5.12 11.435 11.436 11.435 6.315 0 11.434-5.119 11.434-11.435C23.434 5.717 18.316.564 12.001.564zM22.169 15.42l-.062.013-.034-.043c.002-.019.403-1.202.4-2.56-.005-1.463-.599-2.36-1.363-2.36-.465 0-.798.334-.798.831 0 .898 1.097.964 1.097 2.926 0 .798-.166 1.561-.432 2.393-1.23 4.154-5.153 6.082-8.975 6.082-1.763 0-3.014-.361-3.387-.529-.015-.008-.028-.037-.016-.066l.048-.034c.151.06 1.226.397 2.556.397 1.463 0 2.327-.598 2.327-1.33a.823.823 0 0 0-.832-.829c-.898 0-.964 1.13-2.892 1.13-.831 0-1.561-.166-2.427-.432-4.122-1.263-6.087-5.154-6.084-9.01.002-1.878.527-3.372.536-3.388l.059-.01.035.043c-.049.155-.398 1.228-.398 2.556 0 1.463.598 2.327 1.362 2.327a.789.789 0 0 0 .799-.798c0-.898-1.098-.997-1.098-2.925 0-.831.167-1.562.432-2.426 1.265-4.12 5.154-6.052 8.977-6.081 1.776-.013 3.331.518 3.39.565l.011.06-.044.033c-.018-.002-1.029-.425-2.559-.425-1.429-.001-2.327.598-2.327 1.363 0 .432.333.798.831.798.898 0 .964-1.097 2.892-1.097.831 0 1.562.166 2.427.432 4.156 1.261 6.05 5.185 6.084 8.974.018 1.941-.529 3.41-.535 3.42zm-5.715-2.855c-1.13 0-1.995.832-1.995 1.828 0 .831.498 1.495 1.164 1.495.233 0 .465-.133.465-.432 0-.431-.573-.537-.528-1.184.028-.427.43-.711.828-.711.798 0 1.171.774 1.171 1.57-.034 1.23-.938 2.086-2.001 2.086-1.398 0-2.293-1.33-2.293-2.759 0-2.128 1.396-2.959 2.127-3.158.008-.001 1.911.34 1.852-.499-.026-.369-.575-.511-.974-.527-.441-.017-.885.142-.885.142-.233-.117-.393-.346-.492-.611 1.363-1.03 2.326-2.027 2.326-3.158 0-.598-.4-1.131-1.164-1.131-1.363 0-2.394 1.729-2.394 3.291 0 .266 0 .532.067.766-.865.631-1.507 1.023-2.671 1.721 0-.146.031-.521.128-1.008.399-.432.946-1.079.946-1.578 0-.233-.132-.432-.4-.432-.664 0-1.164.998-1.296 1.695-.3.366-.897.832-1.397.832-.399 0-.531-.366-.566-.498 1.263-.432 2.826-2.162 2.826-3.724 0-.333-.133-1.064-1.13-1.064-1.496 0-2.759 2.228-2.759 3.955-.532 0-.731-.565-.731-.996 0-.432.166-.865.166-.997 0-.134-.067-.3-.266-.3-.499 0-.798.664-.798 1.429.034 1.064.732 1.729 1.663 1.795.132.632.698 1.23 1.396 1.23.432 0 .964-.133 1.33-.465-.034.233-.067.431-.1.631-1.463.765-2.527 1.297-3.491 2.161-.762.698-1.195 1.628-1.195 2.359 0 .997.632 1.928 1.928 1.928 1.529 0 2.693-1.23 3.257-2.925.267-.798.373-1.958.44-3.022 1.529-.864 2.254-1.364 3.051-1.931.099.166.2.3.334.399-.699.366-2.36 1.397-2.36 3.823 0 1.729 1.164 3.656 3.457 3.656 1.895 0 3.191-1.562 3.191-3.057 0-1.362-.765-2.625-2.227-2.625zm-9.141 4.653c-.498.023-.83-.296-.83-.827 0-1.429 1.981-2.793 3.477-3.526-.266 1.996-.939 4.275-2.647 4.353zM8.41 9.374c0-1.097 1.082-3.182 1.745-2.962.781.26-.648 2.364-1.745 2.962zm6.283-.499c0-1.362.923-2.688 1.427-2.436.574.287-.43 1.572-1.427 2.436z" /></ svg > } }