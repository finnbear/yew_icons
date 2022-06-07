use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_ch (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-ch" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "#d52b1e" d = "M0 0h640v480H0z" /> { "
    " } < g fill = "#fff" > { "
      " } < path d = "M170 195h300v90H170z" /> { "
      " } < path d = "M275 90h90v300h-90z" /> { "
    " } </ g > { "
  " } </ g > </ svg > } }