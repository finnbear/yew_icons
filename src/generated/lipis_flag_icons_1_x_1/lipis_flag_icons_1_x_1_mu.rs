use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_mu (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-mu" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#009f4d" d = "M0 384h512v128H0z" /> { "
    " } < path fill = "#151f6d" d = "M0 128h512v128H0z" /> { "
    " } < path fill = "#ee2737" d = "M0 0h512v128H0z" /> { "
    " } < path fill = "#ffcd00" d = "M0 256h512v128H0z" /> { "
  " } </ g > </ svg > } }