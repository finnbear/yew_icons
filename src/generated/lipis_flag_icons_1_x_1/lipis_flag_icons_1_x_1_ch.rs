use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_ch (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-ch" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "#d52b1e" d = "M0 0h512v512H0z" /> { "
    " } < g fill = "#fff" > { "
      " } < path d = "M96 208h320v96H96z" /> { "
      " } < path d = "M208 96h96v320h-96z" /> { "
    " } </ g > { "
  " } </ g > </ svg > } }