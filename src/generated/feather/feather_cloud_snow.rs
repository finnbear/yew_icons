use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_cloud_snow (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < path d = "M20 17.58A5 5 0 0 0 18 8h-1.26A8 8 0 1 0 4 16.25" /> { "
  " } < line x1 = "8" y1 = "16" x2 = "8.01" y2 = "16" /> { "
  " } < line x1 = "8" y1 = "20" x2 = "8.01" y2 = "20" /> { "
  " } < line x1 = "12" y1 = "18" x2 = "12.01" y2 = "18" /> { "
  " } < line x1 = "12" y1 = "22" x2 = "12.01" y2 = "22" /> { "
  " } < line x1 = "16" y1 = "16" x2 = "16.01" y2 = "16" /> { "
  " } < line x1 = "16" y1 = "20" x2 = "16.01" y2 = "20" /> </ svg > } }