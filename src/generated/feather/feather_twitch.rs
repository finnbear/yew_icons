use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_twitch (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < path d = "M21 2H3v16h5v4l4-4h5l4-4V2zM11 11V7M16 11V7" /> </ svg > } }