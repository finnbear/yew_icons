use crate :: IconProps ; # [inline (never)] pub fn simple_icons_f_5 (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M12 0C5.373 0 0 5.373 0 12a11.943 11.943 0 002.336 7.1113c.509.004.8594-.111.8984-.33.046-.229.0068-.5825-.0332-.9395a55.067 55.067 0 01-.2344-7.4473c-.609.027-1.1547.055-1.6777.086.02-.471.046-.9188.084-1.3828.517-.05 1.064-.0936 1.666-.1446.026-.406.0568-.7983.0918-1.1953.27-2.43 2.828-3.9162 4.959-4.4902.946-.23 1.5245-.3022 1.9785-.3262.164-.005.3405-.0117.5175-.0117.442 0 .8899.0414 1.1739.2344.46.345.9135.6873 1.3965 1.0683.048.065.1024.1678-.0176.3438-.222.26-.4371.5084-.6621.7754-.13.157-.3454.1164-.5274.0664-.38-.194-.7462-.3728-1.1172-.5528-.672-.299-1.3666-.6061-2.1406-.5761-.483.039-.951.532-1 1.209a101.41 101.41 0 00-.1504 3.2285c1.343-.038 2.6837-.0624 4.0957-.0684l-.002.9453c-.46.206-.8954.413-1.3574.623-.953.011-1.8595.0202-2.7715.0352a125.13 125.13 0 00.1192 7.9317c.024.378.0424.7605.1504 1.0175.13.322.8798.5701 2.5098.6621.007.284.0153.5532.0253.8282-2.655-.077-5.205-.3302-7.248-.6992A11.962 11.962 0 0012 24c6.628 0 12-5.373 12-12a11.942 11.942 0 00-2.0957-6.7754c-.147.607-.2252 1.2378-.3672 1.8828-1.8-.234-3.9131-.4053-6.2871-.4883-.191.602-.3711 1.192-.5781 1.836 3.973.245 5.9048 1.2924 7.0508 2.5254 1.113 1.248 1.3501 2.6262 1.2851 3.9062-.143 2.081-1.0613 3.3971-2.3203 4.3711-1.274.96-2.8139 1.436-4.0469 1.539-1.819.137-4.2515-.2962-4.7695-.6132a178.03 178.03 0 01-.9492-2.2012c-.08-.166-.1294-.3371.0976-.5351.354-.339.6928-.6667 1.0508-1.0137.158-.155.3338-.2991.4668-.0781.489.755.9473 1.4477 1.4063 2.1367.522.77 1.3167 1.4695 3.0527 1.3535 1.459-.13 2.5675-1.2333 2.6855-2.4473.128-2.246-2.1446-3.8396-8.0546-4.3496a2571.27 2571.27 0 013.123-9.371c1.404.065 2.7043.1798 3.9453.3398.919.116 1.772.3287 2.627.4277A11.973 11.973 0 0012 0zm10.0195 21.1113c-.5006 0-.9082.4076-.9082.9082 0 .5007.4076.9082.9082.9082.5007 0 .9082-.4075.9082-.9082 0-.5006-.4075-.9082-.9082-.9082zm0 .127c.4318 0 .7793.3495.7793.7812a.7776.7776 0 01-.7793.7793c-.4317 0-.7812-.3475-.7812-.7793a.7809.7809 0 01.7812-.7812zm-.3652.2773v.9961h.1738v-.3926h.1387c.092 0 .1583.0112.1953.0332.062.037.0938.1146.0938.2286v.08l.002.0293a.07.07 0 01.0039.0118c0 .005-.0001.0068.0039.0098h.162l-.0058-.0137a.106.106 0 01-.0078-.0488.6846.6846 0 01-.004-.0743v-.0742a.276.276 0 00-.0527-.1543c-.037-.053-.0948-.0846-.1718-.0976A.408.408 0 0022.328 22c.066-.042.0977-.1103.0977-.1973 0-.124-.0503-.21-.1543-.252a.752.752 0 00-.2695-.035zm.1738.123h.1504a.45.45 0 01.211.0372c.042.024.0664.0735.0664.1445a.153.153 0 01-.1036.1563.451.451 0 01-.166.0214h-.1582z" /></ svg > } }