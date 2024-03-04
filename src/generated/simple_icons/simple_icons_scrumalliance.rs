use crate :: IconProps ; # [inline (never)] pub fn simple_icons_scrumalliance (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M1.6113 7.6836a.2999.2999 0 1 0 .3398.254.2998.2998 0 0 0-.3398-.254zm.879.002a.325.325 0 0 0-.295.3242.325.325 0 0 0 .3262.3261.325.325 0 0 0 .3242-.3261.325.325 0 0 0-.3242-.3243.325.325 0 0 0-.0313 0zM.8261 8.078a.2746.2746 0 0 0-.2325.3106c.022.15.1608.2524.3106.2304a.2711.2711 0 0 0 .2305-.3086c-.0216-.1493-.1591-.2543-.3086-.2324zm2.086.375a.349.349 0 0 0 .3495.3496.349.349 0 0 0 .3477-.3496.3481.3481 0 0 0-.3487-.3469.3481.3481 0 0 0-.3486.347zm3.9648.0274c-.6133 0-1.17.3394-1.17.9883 0 .9991 1.3438.7344 1.3438 1.4355 0 .3597-.287.6113-.6914.6113-.2958 0-.6265-.0989-.5742-.3926l.0468-.2617h-.4004l-.0605.332c-.0956.5637.5359.7442.9316.7442.6438 0 1.1953-.3976 1.1953-1.0605 0-1.0373-1.3437-.7678-1.3437-1.4356 0-.3362.2964-.545.666-.545.2175 0 .5124.0716.4688.299l-.0391.2323h.4004l.0703-.3613c.0826-.4308-.4827-.586-.8437-.586zm-6.541.287a.2454.2454 0 0 0-.207.2774.2435.2435 0 0 0 .2772.2051.2431.2431 0 0 0 .207-.2754.2457.2457 0 0 0-.2773-.207zm3.3261.127a.3674.3674 0 0 0-.3105.416.3674.3674 0 0 0 .416.3106.3668.3668 0 0 0 .3105-.416.367.367 0 0 0-.416-.3106zm5.664.5332c-.7783 0-1.4003.6805-1.4003 1.457 0 .6254.4032 1.0528 1.0469 1.0528.5787 0 .914-.3555.914-.3555l-.121-.3554s-.3147.3125-.754.3125c-.4087 0-.6562-.2707-.6562-.6875 0-.554.4472-1.0274.9258-1.0274.2087 0 .4523.0852.4218.2793l-.0293.1621h.373l.0528-.2754c.0696-.4262-.4557-.5625-.7734-.5625zm7.625 0c-.4132 0-.7684.369-.873.582h-.0097c.0305-.085.0664-.2712.0664-.328 0-.128-.0662-.1973-.2403-.1973h-.496l-.0645.3554h.2734c.0695 0 .0868.0362.0781.0977l-.3476 1.9434h.4219l.1914-1.0704c.087-.4735.4044-.9667.8261-.9667.2437 0 .3145.1378.3145.332 0 .0805-.0183.1747-.0313.2695l-.2558 1.4356h.42l.1972-1.0957c.0784-.4594.3917-.9414.8008-.9414.2522 0 .3203.1372.3203.3457 0 .2272-.1856 1.0272-.1856 1.25 0 .4072.2867.4492.4824.4492.1043 0 .1875-.0078.1875-.0078l.0645-.3555s-.0446.0098-.1055.0098c-.1087 0-.1953-.0292-.1953-.209 0-.1895.1914-.9814.1914-1.2657 0-.4072-.2293-.6328-.6035-.6328-.3654 0-.6956.2555-.8652.582h-.0098c-.0217-.3788-.2005-.582-.5527-.582zm-4.9628.0274c-.3479 0-.6431.2976-.791.6387h-.0098c.0349-.1138.0781-.3448.0781-.4063 0-.1375-.0647-.2031-.2343-.2031h-.4961l-.0704.3535h.2754c.0739 0 .087.0486.0782.1055l-.3438 1.9375h.4219l.1562-.8907c.0956-.5493.4607-1.0859.8653-1.0859.0565 0 .0956.011.1172.0156l.084-.455a.7813.7813 0 0 0-.131-.0098zm.4922.0293l-.0664.3535h.3652l-.2031 1.166a1.5665 1.5665 0 0 0-.0313.3027c0 .45.2823.6309.6172.6309.4612 0 .7665-.3281.9102-.5742h.0078c-.0304.3599.1437.5254.4394.5254a2.838 2.838 0 0 0 .2012-.0078l.0645-.3555s-.0403.0098-.1055.0098c-.1086 0-.1953-.0386-.1953-.209a.937.937 0 0 1 .0136-.1465l.3047-1.6953h-.787l-.0665.3535h.3653l-.129.7344c-.0784.4545-.4357.9472-.875.9472-.2478 0-.33-.1273-.33-.336 0-.0756.0134-.174.0351-.2733l.252-1.4258zm-12.2871.086a.2255.2255 0 0 0-.1914.2558c.0178.1238.134.209.2578.1914a.2254.2254 0 0 0 .1914-.2559.2277.2277 0 0 0-.2578-.1914zm.2187.7773a.1912.1912 0 0 0-.1621.2168.193.193 0 0 0 .2188.162.1911.1911 0 0 0 .162-.2167.1936.1936 0 0 0-.2187-.1621zm.4941.6484a.166.166 0 0 0-.1406.1875.1642.1642 0 0 0 .1875.1387.1659.1659 0 0 0 .1407-.1875.1644.1644 0 0 0-.1876-.1387zm2.4864.2207c-.222 0-.4024.179-.4024.4004 0 .2219.1805.4023.4024.4023a.4027.4027 0 0 0 .4023-.4023c0-.2214-.1804-.4004-.4023-.4004zm-1.8106.0762a.157.157 0 0 0-.1445.1562.157.157 0 0 0 .1582.1582.157.157 0 0 0 .1563-.1582.157.157 0 0 0-.1563-.1562.1566.1566 0 0 0-.0137 0zm.672.0918a.4355.4355 0 0 0-.418.4355.4355.4355 0 1 0 .4355-.4355.4365.4365 0 0 0-.0176 0zm2.205.1757c-.2024 0-.3652.165-.3652.3672 0 .2028.1628.3672.3652.3672a.3673.3673 0 0 0 .3672-.3672.3677.3677 0 0 0-.3672-.3672zm-3.0645.5098a.4664.4664 0 0 0-.4668.4668c0 .2583.2087.4668.4668.4668a.4663.4663 0 0 0 .4668-.4668.4664.4664 0 0 0-.4668-.4668zm3.8575.3223a.3187.3187 0 0 0-.3184.3203c0 .1774.1414.3203.3184.3203a.32.32 0 0 0 .3203-.3203.3205.3205 0 0 0-.3203-.3203zm2.8125.4824l-1.5743 3.0547h-.3086l-.0488.2754h.9785l.0528-.2754h-.3477l.4395-.8574h1.1914l.1347.8574h-.3535l-.0469.2754h.9649l.0488-.2754h-.3086l-.4844-3.0547zm1.6835 0l-.0488.2656h.3926l-.4395 2.4668c-.013.0663-.0214.1323-.0214.1797 0 .3742.2561.4316.4257.4316.0697 0 .1524-.0136.1524-.0136l.0469-.2754s-.0334.0098-.1075.0098c-.1043 0-.209-.014-.209-.213 0-.0378.0089-.0995.0176-.1562l.4785-2.6953zm1.1622 0l-.0489.2656h.3907l-.4375 2.4668c-.0132.0663-.0235.1323-.0235.1797 0 .3742.2581.4316.4277.4316.0697 0 .1524-.0136.1524-.0136l.0469-.2754s-.0336.0098-.1075.0098c-.1045 0-.209-.014-.209-.213 0-.0378.0089-.0995.0176-.1562l.4766-2.6953zm1.6386 0l-.0742.4023h.336l.0742-.4023zM.957 13.1738a.4928.4928 0 0 0-.4922.4922c0 .2718.2208.4902.4922.4902a.4905.4905 0 0 0 .4922-.4902.4925.4925 0 0 0-.4922-.4922zm22.5762.039c-.2566 0-.4629.2294-.4629.5177 0 .288.2063.5175.4629.5175.2592 0 .4668-.2296.4668-.5175 0-.2883-.2076-.5176-.4668-.5176zm-15.3555.004h.0078s.0003.104.0176.2129l.209 1.3633H7.3906l.7031-1.3633c.0523-.109.084-.2129.084-.2129zm15.3555.0918c.2097 0 .3711.1847.3711.4219 0 .237-.1614.4218-.3711.4218-.2073 0-.3672-.1848-.3672-.4218 0-.2372.1599-.422.3672-.422zm-17.9453.0976a.2965.2965 0 0 0-.2969.297c0 .164.1326.2968.2969.2968s.2988-.1328.2988-.2969c0-.1643-.1345-.2968-.2988-.2968zm17.7871.0547v.5352h.0918v-.207h.08l.0919.207h.1015l-.0937-.1973c-.0117-.0242-.0176-.0293-.0176-.0293v-.0039c.0404-.0112.0898-.0583.0898-.1406 0-.0993-.0617-.164-.1542-.164zm.0918.0762h.0781c.0482 0 .0801.0327.0801.0879 0 .0568-.0319.0898-.08.0898h-.0782zm-9.1602.2285c-.4568 0-.7949.2461-.7949.2461l.0899.2617s.2971-.2226.6582-.2226c.3349 0 .4472.1746.4472.4258 0 .085-.0351.2695-.0351.2695h-.1426c-.6003 0-1.4844.1555-1.4844.918 0 .3836.2884.5976.6406.5976.5177 0 .8091-.5117.8047-.5117h.0078c-.0217.3174.14.4648.3965.4648.0783 0 .164-.0097.164-.0097l.049-.2656s-.0393.0097-.1134.0097c-.1 0-.205-.024-.205-.2226 0-.199.2187-1.0175.2187-1.297 0-.4593-.2836-.664-.7012-.664zm2.92 0c-.5395 0-.8878.4272-.9746.6309h-.0098c.0393-.0995.0742-.327.0742-.379 0-.1231-.0554-.1952-.2207-.1952h-.418l-.0488.2656h.2969c.0695 0 .0867.0467.0781.1035l-.3613 2.0137h.2949l.1973-1.0938c.0956-.5587.53-1.043 1-1.043.2435 0 .3789.1285.3789.3985 0 .2652-.2051 1.1269-.2051 1.3398 0 .36.2613.4082.418.4082.0738 0 .1601-.0097.1601-.0097l.0488-.2656s-.0397.0097-.1093.0097c-.1042 0-.209-.019-.209-.2226 0-.1895.205-1.0562.205-1.3496 0-.393-.2129-.6114-.5956-.6114zm2.5312 0c-.7264 0-1.3789.6491-1.3789 1.459 0 .6394.4078 1.0371 1.004 1.0371.5567 0 .8886-.3555.8886-.3555l-.0957-.2656s-.3127.332-.7695.332c-.4262 0-.7227-.2891-.7227-.7675 0-.6487.5301-1.1504 1.039-1.1504.2132 0 .5096.09.4747.3125l-.0313.1758h.2754l.043-.252c.0652-.3881-.4308-.5254-.7266-.5254zm2.3106 0c-.7395 0-1.2793.7156-1.2793 1.4922 0 .6015.392 1.004.9921 1.004.461 0 .8223-.3126.8223-.3126l-.0918-.2656s-.323.289-.7187.289c-.461 0-.6992-.3262-.6992-.7382 0-.1185.0214-.2188.0214-.2188h1.6797s.0606-.204.0606-.3886c0-.4973-.2477-.8614-.7871-.8614zm-10.0586.0567l-.047.2656h.3907l-.2773 1.5351c-.0088.0664-.0235.1265-.0235.1739 0 .3742.2624.418.4278.418.0608 0 .1523-.0098.1523-.0098l.0469-.2656s-.0355.0097-.1094.0097c-.1045 0-.207-.0238-.207-.2226 0-.0379.003-.0846.0117-.1368l.3223-1.7675zm10.0449.2129c.2873 0 .5.1855.5.5644 0 .0948-.0176.1797-.0176.1797H21.168c.1565-.4833.526-.7441.8867-.7441zm-16.6035.4238c-.139 0-.252.113-.252.252 0 .1387.113.25.252.25s.252-.1113.252-.25c0-.139-.113-.252-.252-.252zm9.0508.5293h.127l-.0216.1035c-.0954.412-.4389.9043-.8476.9043-.2916 0-.4141-.1753-.4141-.379 0-.5872.7908-.6288 1.1563-.6288zm-9.5684.4043c-.1198 0-.2188.0966-.2188.2168a.22.22 0 0 0 .2188.2187c.12 0 .2168-.0988.2168-.2187a.2164.2164 0 0 0-.2168-.2168zm-.8086.5137a.2077.2077 0 0 0-.207.207c0 .114.0932.205.207.205a.2044.2044 0 0 0 .205-.205.2062.2062 0 0 0-.205-.207Z" /></ svg > } }