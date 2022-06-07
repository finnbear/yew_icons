use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_td (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-td" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#000067" d = "M0 0h171.2v512H0z" /> { "
    " } < path fill = "red" d = "M340.8 0H512v512H340.8z" /> { "
    " } < path fill = "#ff0" d = "M171.2 0h169.6v512H171.2z" /> { "
  " } </ g > </ svg > } }