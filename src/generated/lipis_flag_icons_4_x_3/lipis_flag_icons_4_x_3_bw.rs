use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_bw (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-bw" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#00cbff" d = "M0 0h640v480H0z" /> { "
    " } < path fill = "#fff" d = "M0 160h640v160H0z" /> { "
    " } < path d = "M0 186h640v108H0z" /> { "
  " } </ g > </ svg > } }