use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_tenge_sign (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 384 512" >< path d = "M0 64C0 46.33 14.33 32 32 32H352C369.7 32 384 46.33 384 64C384 81.67 369.7 96 352 96H32C14.33 96 0 81.67 0 64zM0 192C0 174.3 14.33 160 32 160H352C369.7 160 384 174.3 384 192C384 209.7 369.7 224 352 224H224V448C224 465.7 209.7 480 192 480C174.3 480 160 465.7 160 448V224H32C14.33 224 0 209.7 0 192z" /></ svg > } }