use crate :: IconProps ; # [inline (never)] pub fn simple_icons_delonghi (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M20.16 8.158H3.841A3.85 3.85 0 000 11.999a3.848 3.848 0 003.841 3.843H20.16A3.844 3.844 0 0024 11.999a3.846 3.846 0 00-3.84-3.841zm-.051 7.409H3.885A3.564 3.564 0 01.319 12a3.564 3.564 0 013.566-3.571h16.224A3.566 3.566 0 0123.68 12a3.564 3.564 0 01-3.571 3.567zm-3.84-3.864c.084 0 .142.013.177.043v1.034a.836.836 0 01-.177.043c-.062-.003-.107-.03-.133-.084a.671.671 0 01-.036-.252v-.544c0-.16.059-.24.169-.24zm-4.635.137v.829a.392.392 0 01-.036.195c-.026.035-.066.053-.119.053-.053 0-.097-.017-.124-.049-.023-.035-.036-.102-.036-.198V11.84c0-.147.053-.217.155-.217.107 0 .16.07.16.217zm-4.9-.062a.802.802 0 01.014.159h-.346c0-.062.005-.115.014-.164 0-.022.008-.044.013-.066.018-.058.067-.084.137-.084.08 0 .129.026.151.084.004.022.013.044.017.071zM4.24 11.02c.045.039.067.119.067.234v1.224c0 .115-.018.191-.054.226a.287.287 0 01-.177.062h-.12v-1.803h.08c.093 0 .16.018.204.057zm15.892-1.723H3.881a2.71 2.71 0 00-2.702 2.702 2.712 2.712 0 002.702 2.703h16.243a2.71 2.71 0 002.697-2.703c0-1.475-1.192-2.702-2.689-2.702zM5.676 10.574H7.55v.248H5.676v-.248zm-.39 1.288v.7c0 .15-.028.292-.085.42a.668.668 0 01-.302.325c-.142.084-.336.127-.58.127H2.84v-.562c.057-.023.093-.058.107-.103a.665.665 0 00.022-.185v-1.48a.488.488 0 00-.026-.181c-.013-.04-.049-.071-.102-.093v-.557h1.48c.222 0 .403.03.544.096.142.063.253.16.32.289.066.133.103.298.103.492v.712zm2.295.625H6.402v.209a.5.5 0 00.027.181c.023.044.067.072.137.072.071 0 .116-.023.133-.067a.469.469 0 00.031-.177v-.085h.851v.568c-.084.079-.226.15-.429.212a1.98 1.98 0 01-1.193 0 .597.597 0 01-.324-.27.934.934 0 01-.097-.456v-.895a.905.905 0 01.128-.417.581.581 0 01.329-.221c.146-.039.336-.062.571-.062.222 0 .399.018.537.057a.729.729 0 01.306.172c.07.076.115.159.137.254.013.066.027.137.031.217.005.039.005.084.005.127v.581zm2.644.947H7.922v-.562c.076-.045.111-.124.111-.239V11.059c0-.11-.036-.185-.111-.23v-.557h1.205v.557a.21.21 0 00-.088.107.575.575 0 00-.031.19.826.826 0 01.008.123v1.516h.124c.093-.009.155-.035.181-.089a.581.581 0 00.041-.252v-.522h.863v1.532zm2.274-1.165v.416a.909.909 0 01-.107.47.602.602 0 01-.319.258 1.58 1.58 0 01-.549.079c-.271-.005-.483-.031-.634-.084a.565.565 0 01-.332-.257c-.067-.119-.102-.279-.102-.487V11.853a.89.89 0 01.094-.434c.057-.116.164-.2.314-.258.151-.058.355-.084.611-.084.249 0 .448.026.594.071a.584.584 0 01.324.239c.071.115.107.274.107.479v.403zm2.499 1.165h-.922a1.142 1.142 0 01-.031-.225 2.403 2.403 0 01-.014-.235c-.004-.075-.004-.168-.004-.279v-.775a.195.195 0 00-.204-.204c-.04 0-.08.011-.11.023v.878c0 .088.004.153.013.195.013.039.04.075.088.106v.518h-1.076v-.518a.226.226 0 00.093-.096.414.414 0 00.017-.143v-.771a.59.59 0 00-.017-.172.242.242 0 00-.093-.102v-.528h.935v.151c.194-.137.389-.208.584-.208.182.005.319.044.412.119.094.076.155.16.181.253a.95.95 0 01.041.265v1.001c0 .053.009.098.018.137a.176.176 0 00.089.09v.52zm2.419-1.807a.187.187 0 00-.094.102.7.7 0 00-.017.164V13.487c0 .201-.027.368-.08.496a.597.597 0 01-.305.299c-.156.067-.373.101-.656.101a2.109 2.109 0 01-.492-.062 1.66 1.66 0 01-.47-.2v-.571h.797c0 .172.059.256.178.256.075-.003.12-.034.138-.088a.837.837 0 00.03-.234v-.169a.926.926 0 01-.527.159c-.235-.003-.408-.066-.518-.191-.106-.123-.164-.296-.164-.518v-.899c0-.244.048-.44.147-.581.094-.138.28-.209.545-.209.092 0 .19.013.288.045a.738.738 0 01.253.124v-.142h.949v.524zm2.503 1.807h-.917a4.698 4.698 0 01-.053-.553c.009-.066.009-.159.009-.274v-.643a.257.257 0 00-.062-.172.21.21 0 00-.155-.076.275.275 0 00-.097.023v.966c0 .111.031.186.097.213v.518H17.67v-.518a.225.225 0 00.093-.102.668.668 0 00.017-.187v-1.515c0-.195-.039-.307-.119-.324v-.517h.983v.947a.82.82 0 01.266-.128.863.863 0 01.27-.044.77.77 0 01.452.146c.124.094.19.253.19.483v.896a.431.431 0 01-.008.075.645.645 0 00.026.181c.009.032.04.062.08.085v.52zm.389-3.091a.526.526 0 01.328-.102c.084 0 .164.017.234.045a.36.36 0 01.173.124.294.294 0 01.067.19v.008a.317.317 0 01-.063.191.46.46 0 01-.172.124.675.675 0 01-.24.04.504.504 0 01-.323-.098.324.324 0 01-.004-.522zm.913 3.091h-1.081v-.518c.053-.035.084-.067.092-.106a.628.628 0 00.018-.183v-.743a.522.522 0 00-.018-.159.162.162 0 00-.092-.099v-.523h.974v1.604c0 .103.036.169.106.209v.518z" /></ svg > } }