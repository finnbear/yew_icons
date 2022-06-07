use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_mu (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-mu" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#00a04d" d = "M0 360h640v120H0z" /> { "
    " } < path fill = "#151f6d" d = "M0 120h640v120H0z" /> { "
    " } < path fill = "#ee2737" d = "M0 0h640v120H0z" /> { "
    " } < path fill = "#ffcd00" d = "M0 240h640v120H0z" /> { "
  " } </ g > </ svg > } }