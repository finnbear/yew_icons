use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_tt (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-tt" viewBox = "0 0 640 480" > < path fill = "#fff" d = "M0 0h640v480H0z" /> { "
  " } < path fill = "#e00000" fill - rule = "evenodd" d = "M463.7 480 0 1v478.8l463.7.2zM176.3 0 640 479V.2L176.3 0z" /> { "
  " } < path fill - rule = "evenodd" d = "M27.7.2h118.6l468.2 479.3H492.2L27.7.2z" /> </ svg > } }