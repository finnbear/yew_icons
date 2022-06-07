use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_ml (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-ml" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" > { "
    " } < path fill = "red" d = "M425.8 0H640v480H425.7z" /> { "
    " } < path fill = "#009a00" d = "M0 0h212.9v480H0z" /> { "
    " } < path fill = "#ff0" d = "M212.9 0h214v480h-214z" /> { "
  " } </ g > </ svg > } }