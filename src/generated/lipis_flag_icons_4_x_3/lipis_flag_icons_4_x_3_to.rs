use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_to (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-to" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "#c10000" d = "M0 0h640v480H0z" /> { "
    " } < path fill = "#fff" d = "M0 0h250v200.3H0z" /> { "
    " } < g fill = "#c10000" > { "
      " } < path d = "M102.8 31.2h39.9v139.6h-39.8z" /> { "
      " } < path d = "M192.6 81v40H53V81z" /> { "
    " } </ g > { "
  " } </ g > </ svg > } }