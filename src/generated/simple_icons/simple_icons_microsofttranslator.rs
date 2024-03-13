use crate :: IconProps ; # [inline (never)] pub fn simple_icons_microsofttranslator (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M3.2 0A3.193 3.193 0 0 0 0 3.2v17.6C0 22.573 1.427 24 3.2 24h17.6c1.773 0 3.2-1.427 3.2-3.2V3.2C24 1.427 22.573 0 20.8 0H3.2Zm11.113 4.002.049.002c1.052.209 1.438.376 1.438.62 0 .01-.004.034-.008.044l-.058.19c-.043.137-.11.345-.183.606.3-.106.556-.242.766-.407.426-.332.808-.437 1.059-.297a.411.411 0 0 1 .2.435c-.033.277-.322.671-1.017.86-.422.115-.821.2-1.215.266a9.147 9.147 0 0 0-.192 1.58 5.61 5.61 0 0 1 1.03-.266c-.112-.105-.328-.223-.554-.346-.06-.034-.205-.112-.205-.26 0-.016.006-.062.014-.078.04-.136.203-.152.322-.152.056 0 .128.003.216.015.68.033 1.102.19 1.288.474a.535.535 0 0 1 .088.296v.013c.766.081 1.412.374 1.83.836.333.369.509.832.508 1.34 0 .042-.003.084-.006.127-.12 2.178-2.942 2.996-3.062 3.03a.253.253 0 0 1-.052.007.35.35 0 0 1-.067-.012c-.102-.04-.306-.12-.306-.303.012-.12.08-.239.336-.268-.003-.003 2.311-.877 2.311-2.3 0-.059-.004-.12-.01-.183-.142-1.017-.705-1.548-1.722-1.621-.416.959-.862 1.766-1.327 2.404.145.223.307.422.482.586a.195.195 0 0 1 .035.237.446.446 0 0 1-.317.218c-.203.033-.421-.072-.657-.306-.026-.028-.055-.057-.08-.087-.822.867-1.535 1.053-1.992 1.053-.336 0-.552-.099-.61-.128-.258-.133-.54-.556-.542-1.17 0-.62.285-1.792 2.188-2.78a10.019 10.019 0 0 1-.013-.485c0-.411.025-.87.068-1.366a8.057 8.057 0 0 1-1.19-.033.223.223 0 0 1-.07-.022 1.708 1.708 0 0 1-.578-.483l-.08-.095c-.087-.1-.186-.215-.134-.361.019-.053.075-.144.237-.167-.005 0 .24-.041.432.11.05.037.09.083.136.133.083.094.143.163.216.172.065.009.297.034.622.034.165 0 .328-.008.488-.02.037-.306.086-.63.142-.967a.396.396 0 0 0-.219-.209c-.157-.062-.32-.128-.32-.287 0-.01.002-.032.005-.04.019-.158.188-.178.28-.19Zm-8.97 1.772 1.648 1.652c1.683-1.16 3.887-.807 5.013-.066-1.206.088-2.59.802-3.239 1.846l1.808 1.813h-5.23V5.774Zm10.812 2.533a5.66 5.66 0 0 0-.995.269c.027.414.097.802.205 1.156.39-.605.681-1.158.765-1.366.01-.02.017-.04.025-.059Zm-1.807.674c-.757.528-1.413 1.519-1.413 2.178l.001.059c.014.227.117.375.323.458.325.128.868-.271 1.512-1.099-.204-.435-.346-.971-.423-1.596Zm-6.47 3.434c.823 0 1.458.17 1.907.512.45.34.674.923.674 1.744 0 .216-.012.613-.032 1.184-.02.574-.032.955-.032 1.146.001.845.064 1.597.189 2.256l-1.089.041a2.738 2.738 0 0 1-.083-.647h-.07c-.57.52-1.343.78-2.325.78-.532 0-.99-.156-1.374-.469-.383-.314-.576-.878-.576-1.459 0-.715.196-1.33.744-1.832.654-.598 2.016-.868 3.509-.458.108.03.022-.24.022-.383 0-.571-.153-.952-.455-1.142-.302-.19-.762-.285-1.377-.285-.728 0-.666.147-1.37.327l-.177-.96c.695-.199 1.012-.355 1.914-.355Zm5.282 1.806h5.228v5.246l-1.645-1.652c-1.685 1.16-3.888.806-5.015.065 1.206-.087 2.591-.8 3.241-1.845L13.16 14.22Zm-5.038 1.624a2.57 2.57 0 0 0-.895.173c-.551.24-1.056.558-1.03 1.373.019.63.445 1.086 1.334 1.086.762 0 1.37-.191 1.825-.571a58.944 58.944 0 0 1-.036-1.9 3.685 3.685 0 0 0-1.198-.16Z" /></ svg > } }