use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_face_sad_cry (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 512 512" >< path d = "M352 493.4C322.4 505.4 289.9 512 256 512C222.1 512 189.6 505.4 160 493.4V288C160 279.2 152.8 272 144 272C135.2 272 128 279.2 128 288V477.8C51.48 433.5 0 350.8 0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 350.8 460.5 433.5 384 477.8V288C384 279.2 376.8 272 368 272C359.2 272 352 279.2 352 288V493.4zM217.6 236.8C224.7 231.5 226.1 221.5 220.8 214.4C190.4 173.9 129.6 173.9 99.2 214.4C93.9 221.5 95.33 231.5 102.4 236.8C109.5 242.1 119.5 240.7 124.8 233.6C142.4 210.1 177.6 210.1 195.2 233.6C200.5 240.7 210.5 242.1 217.6 236.8zM316.8 233.6C334.4 210.1 369.6 210.1 387.2 233.6C392.5 240.7 402.5 242.1 409.6 236.8C416.7 231.5 418.1 221.5 412.8 214.4C382.4 173.9 321.6 173.9 291.2 214.4C285.9 221.5 287.3 231.5 294.4 236.8C301.5 242.1 311.5 240.7 316.8 233.6zM208 368C208 394.5 229.5 416 256 416C282.5 416 304 394.5 304 368V336C304 309.5 282.5 288 256 288C229.5 288 208 309.5 208 336V368z" /></ svg > } }