use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_sj (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-sj" viewBox = "0 0 640 480" > < path fill = "#ef2b2d" d = "M0 0h640v480H0z" /> { "
  " } < path fill = "#fff" d = "M180 0h120v480H180z" /> { "
  " } < path fill = "#fff" d = "M0 180h640v120H0z" /> { "
  " } < path fill = "#002868" d = "M210 0h60v480h-60z" /> { "
  " } < path fill = "#002868" d = "M0 210h640v60H0z" /> </ svg > } }