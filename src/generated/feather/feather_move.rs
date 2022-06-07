use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_move (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < polyline points = "5 9 2 12 5 15" /> { "
  " } < polyline points = "9 5 12 2 15 5" /> { "
  " } < polyline points = "15 19 12 22 9 19" /> { "
  " } < polyline points = "19 9 22 12 19 15" /> { "
  " } < line x1 = "2" y1 = "12" x2 = "22" y2 = "12" /> { "
  " } < line x1 = "12" y1 = "2" x2 = "12" y2 = "22" /> </ svg > } }