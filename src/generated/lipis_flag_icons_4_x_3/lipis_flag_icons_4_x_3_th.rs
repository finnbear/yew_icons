use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_th (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-th" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#f4f5f8" d = "M0 0h640v480H0z" /> { "
    " } < path fill = "#2d2a4a" d = "M0 162.5h640v160H0z" /> { "
    " } < path fill = "#a51931" d = "M0 0h640v82.5H0zm0 400h640v80H0z" /> { "
  " } </ g > </ svg > } }