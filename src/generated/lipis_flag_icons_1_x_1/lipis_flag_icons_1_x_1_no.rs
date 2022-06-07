use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_no (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-no" viewBox = "0 0 512 512" > < path fill = "#ed2939" d = "M0 0h512v512H0z" /> { "
  " } < path fill = "#fff" d = "M128 0h128v512H128z" /> { "
  " } < path fill = "#fff" d = "M0 192h512v128H0z" /> { "
  " } < path fill = "#002664" d = "M160 0h64v512h-64z" /> { "
  " } < path fill = "#002664" d = "M0 224h512v64H0z" /> </ svg > } }