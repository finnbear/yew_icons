use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_th (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-th" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#f4f5f8" d = "M0 0h512v512H0z" /> { "
    " } < path fill = "#2d2a4a" d = "M0 173.4h512V344H0z" /> { "
    " } < path fill = "#a51931" d = "M0 0h512v88H0zm0 426.7h512V512H0z" /> { "
  " } </ g > </ svg > } }