use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_vn (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } id = "flag-icons-vn" viewBox = "0 0 640 480" > < defs > { "
    " } < clipPath id = "vn-a" > { "
      " } < path fill - opacity = ".7" d = "M-85.3 0h682.6v512H-85.3z" /> { "
    " } </ clipPath > { "
  " } </ defs > { "
  " } < g fill - rule = "evenodd" transform = "translate(80) scale(.9375)" > { "
    " } < path fill = "#ec0015" d = "M-128 0h768v512h-768z" /> { "
    " } < path fill = "#ff0" d = "M349.6 381 260 314.3l-89 67.3L204 272l-89-67.7 110.1-1 34.2-109.4L294 203l110.1.1-88.5 68.4 33.9 109.6z" /> { "
  " } </ g > </ svg > } }