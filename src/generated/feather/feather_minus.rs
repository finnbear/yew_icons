use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_minus (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < line x1 = "5" y1 = "12" x2 = "19" y2 = "12" /> </ svg > } }