use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_dice_three (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 448 512" >< path d = "M384 32H64C28.62 32 0 60.62 0 96v320c0 35.38 28.62 64 64 64h320c35.38 0 64-28.62 64-64V96C448 60.62 419.4 32 384 32zM128 192C110.4 192 96 177.6 96 160s14.38-32 32-32s32 14.38 32 32S145.6 192 128 192zM224 288C206.4 288 192 273.6 192 256s14.38-32 32-32s32 14.38 32 32S241.6 288 224 288zM320 384c-17.62 0-32-14.38-32-32s14.38-32 32-32s32 14.38 32 32S337.6 384 320 384z" /></ svg > } }