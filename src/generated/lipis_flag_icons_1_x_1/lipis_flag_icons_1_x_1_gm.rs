use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_gm (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-gm" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "red" d = "M0 0h512v170.7H0z" /> { "
    " } < path fill = "#fff" d = "M0 170.7h512V199H0z" /> { "
    " } < path fill = "#009" d = "M0 199.1h512V313H0z" /> { "
    " } < path fill = "#fff" d = "M0 312.9h512v28.4H0z" /> { "
    " } < path fill = "#090" d = "M0 341.3h512V512H0z" /> { "
  " } </ g > </ svg > } }