use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_trending_down (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < polyline points = "23 18 13.5 8.5 8.5 13.5 1 6" /> { "
  " } < polyline points = "17 18 23 18 23 12" /> </ svg > } }