use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_file_text (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < path d = "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" /> { "
  " } < polyline points = "14 2 14 8 20 8" /> { "
  " } < line x1 = "16" y1 = "13" x2 = "8" y2 = "13" /> { "
  " } < line x1 = "16" y1 = "17" x2 = "8" y2 = "17" /> { "
  " } < polyline points = "10 9 9 9 8 9" /> </ svg > } }