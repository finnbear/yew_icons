use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_mm (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-mm" viewBox = "0 0 640 480" > < defs > { "
    " } < path id = "a" fill = "#fff" d = "m0-.5.2.5h-.4z" transform = "scale(8.844)" /> { "
    " } < g id = "b" > { "
      " } < use href = "#a" width = "18" height = "12" transform = "rotate(-144)" /> { "
      " } < use href = "#a" width = "18" height = "12" transform = "rotate(-72)" /> { "
      " } < use href = "#a" width = "18" height = "12" /> { "
      " } < use href = "#a" width = "18" height = "12" transform = "rotate(72)" /> { "
      " } < use href = "#a" width = "18" height = "12" transform = "rotate(144)" /> { "
    " } </ g > { "
  " } </ defs > { "
  " } < path fill = "#fecb00" d = "M0-.1h640V160H0z" /> { "
  " } < path fill = "#ea2839" d = "M0 320h640v160H0z" /> { "
  " } < path fill = "#34b233" d = "M0 160h640v160H0z" /> { "
  " } < use href = "#b" width = "18" height = "12" x = "9" y = "6.4" transform = "matrix(40 0 0 40 -40 0)" /> </ svg > } }