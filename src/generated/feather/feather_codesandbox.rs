use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_codesandbox (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < path d = "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" /> { "
  " } < polyline points = "7.5 4.21 12 6.81 16.5 4.21" /> { "
  " } < polyline points = "7.5 19.79 7.5 14.6 3 12" /> { "
  " } < polyline points = "21 12 16.5 14.6 16.5 19.79" /> { "
  " } < polyline points = "3.27 6.96 12 12.01 20.73 6.96" /> { "
  " } < line x1 = "12" y1 = "22.08" x2 = "12" y2 = "12" /> </ svg > } }