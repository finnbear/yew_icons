use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_cz (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-cz" viewBox = "0 0 640 480" > < path fill = "#fff" d = "M0 0h640v240H0z" /> { "
  " } < path fill = "#d7141a" d = "M0 240h640v240H0z" /> { "
  " } < path fill = "#11457e" d = "M360 240 0 0v480z" /> </ svg > } }