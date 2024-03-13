use crate :: IconProps ; # [inline (never)] pub fn simple_icons_kicad (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M7.4668 7.3039c-.472.0238-.8477.4142-.8477.8906 0 .4918.3994.8906.8926.8906.4933 0 .8926-.3988.8926-.8906 0-.4918-.3993-.8906-.8926-.8906-.0154 0-.0297-.0008-.0449 0zM.25 8.0109c-.1394 0-.25.1214-.25.254v8.1777c0 .1325.1106.2539.25.2539h8.5215c.1394 0 .248-.1214.248-.254V8.2649c0-.1327-.1083-.2539-.248-.2539h-.2598c.011.0595.0176.121.0176.1836 0 .0605-.0054.1201-.0156.1777h.1445v7.963H.3613v-7.963h6.1485a1.0179 1.0179 0 0 1-.0157-.1777c0-.0628.0066-.1238.0176-.1836zm.2617.5117v7.664h7.9961v-7.664h-.0332a1.025 1.025 0 0 1-.4883.5703.8482.8482 0 0 1-.4746.1426.8483.8483 0 0 1-.4746-.1426 1.025 1.025 0 0 1-.4883-.5703zm21.5606.252c.0966.1084.1562.2266.1797.3555.0116.0615.0175.1992.0175.4101v1.664c-.2753-.2606-.6554-.3906-1.1386-.3906-.3662 0-.6896.1007-.9707.3028-.5946.4276-.8926 1.2249-.8926 2.3906 0 .3515.0469.67.1406.957.1347.41.3487.7335.6387.9707.3075.252.669.379 1.0879.379.495 0 .9034-.1574 1.2226-.4708v.373H24c-.1347-.1492-.2012-.4072-.2012-.7733v-6.168Zm-9.6328.2988c-.785 0-1.463.2873-2.0372.8613-.6209.621-.9316 1.4593-.9316 2.5137 0 .9402.253 1.7238.7598 2.3535.5447.6795 1.2746 1.0196 2.1914 1.0196.577 0 1.0987-.131 1.5644-.3946.2636-.1494.414-.22.4492-.211l-.7304-1.1952c-.3456.3807-.7404.5703-1.1856.5703-.249 0-.4834-.0782-.703-.2363-.4717-.3398-.7071-.9749-.7071-1.9063 0-.328.0322-.628.0996-.9004.205-.8318.6447-1.248 1.3183-1.248.4306 0 .7755.1641 1.0332.4922l.7872-1.1426c-.085-.009-.1964-.0508-.334-.127-.536-.2987-1.0617-.4492-1.5742-.4492zM.5605 9.175H2.625c-.164.164-.2461.4474-.246.8515v1.6133l1.2616-1.5957c.2578-.325.3867-.5674.3867-.7285 0-.0585-.0088-.1054-.0234-.1406h2.2012c-.167.0937-.375.2901-.627.5918-.0673.079-.1856.2247-.3554.4355L3.5703 12.259l1.9727 2.7148c.12.164.2725.3497.457.5547.0498.0527.1211.1144.2148.1875H3.957a.7156.7156 0 0 0 .0254-.1797c0-.1611-.1114-.3946-.334-.6992L2.379 13.1066v1.754c0 .407.082.6914.2461.8554H.5605c.1143-.1142.1866-.2442.2188-.3906.0175-.082.0273-.2355.0273-.461v-4.8379c0-.2255-.0098-.3789-.0273-.4609-.0322-.1464-.1045-.2764-.2188-.3906zm16.2032 1.6386c-.2373 0-.4561.0195-.6582.0605l-.5977.1504c-.2812.0703-.4717.1055-.5683.1055l.4355 1.0488c.41-.2167.8019-.3242 1.1738-.3242.4364 0 .6543.2511.6543.752-.2167-.0116-.3691-.0176-.457-.0176-.618 0-1.0987.083-1.4414.25-.5946.29-.8906.7384-.8906 1.3476 0 .2666.0469.506.1406.7168.1318.2959.3409.5244.6308.6856.2754.1522.5831.2276.92.2246.4745-.006.8623-.1632 1.164-.4707v.373h1.6387c-.0996-.1083-.1621-.2276-.1855-.3594-.0117-.0644-.0176-.2031-.0176-.414V12.794c0-.4305-.0587-.791-.1758-1.084-.2402-.5974-.8284-.8964-1.7656-.8964zm-10.211.0957h1.7266v4.0332c0 .211.0059.3497.0176.4141.0234.1318.086.251.1855.3594H6.5488c.0996-.1083.1602-.2276.1836-.3594.0117-.0644.0176-.2032.0176-.414V11.675c0-.2109-.0059-.3467-.0176-.4082-.0234-.1289-.083-.249-.1797-.3574zM21.623 11.929c.2373 0 .4532.082.6465.246v2.1934c-.2284.1845-.4668.2754-.7129.2754-.4715 0-.707-.4025-.707-1.211 0-1.0016.258-1.5038.7734-1.5038zm-4.793 1.6484c.123 0 .2403.009.3516.0293v.7617c-.1464.252-.38.377-.6992.377-.1728 0-.3154-.046-.4297-.1367-.126-.0996-.1894-.2315-.1894-.3985 0-.2226.1114-.3906.334-.502.1757-.0878.3867-.1308.6328-.1308z" /></ svg > } }