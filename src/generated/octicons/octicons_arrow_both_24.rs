use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn octicons_arrow_both_24 (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/primer/octicons - (c) GitHub, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 24 24" >< path d = "M7.78 5.97a.75.75 0 00-1.06 0l-5.25 5.25a.75.75 0 000 1.06l5.25 5.25a.75.75 0 001.06-1.06L3.81 12.5h16.38l-3.97 3.97a.75.75 0 101.06 1.06l5.25-5.25a.75.75 0 000-1.06l-5.25-5.25a.75.75 0 10-1.06 1.06L20.19 11H3.81l3.97-3.97a.75.75 0 000-1.06z" /></ svg > } }