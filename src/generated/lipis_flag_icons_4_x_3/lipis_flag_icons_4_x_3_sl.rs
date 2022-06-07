use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_sl (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-sl" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#0000cd" d = "M0 320.3h640V480H0z" /> { "
    " } < path fill = "#fff" d = "M0 160.7h640v159.6H0z" /> { "
    " } < path fill = "#00cd00" d = "M0 0h640v160.7H0z" /> { "
  " } </ g > </ svg > } }