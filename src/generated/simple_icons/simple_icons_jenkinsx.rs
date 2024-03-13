use crate :: IconProps ; # [inline (never)] pub fn simple_icons_jenkinsx (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M11.9988 3.4123c-.0652 0-.1328 0-.198.0024-1.7862.041-3.5242.5914-3.954 1.3566-1.7138.3259-3.1573 1.4314-3.9442 2.9304l-2.298-2.426-.1183 1.3977C1.31 6.35.4773 5.1286.0235 4.3778c-.1231.4442.2728 3.8864.3814 4.3306.1062.4369.8255 3.4398 1.376 3.389.0772-.0289.2244-.6348.4078-1.178l1.2601 2.344c.5866 2.3414 2.4863 4.1736 4.8616 4.666-.0507.2897-.0531.5866-.0338.8425.0507.6372.3017 1.6535 1.0428 1.7983a.8484.8484 0 00.3814-.0145c.4803-.128 1.021-.3982 1.5087-.6976.1255.058.263.0941.4103.0941h.7628a.973.973 0 00.4104-.0917c.4851.2994 1.0283.5673 1.5062.6977a.8482.8482 0 00.3814.0144c.741-.1448.9921-1.161 1.0428-1.7983.0193-.2535.017-.5528-.0338-.8425 2.3753-.4924 4.275-2.3246 4.8616-4.666l1.26-2.344c.1835.5432.3332 1.1467.408 1.178.5504.0508 1.2697-2.9521 1.376-3.389.1085-.4466.5044-3.8912.3813-4.333-.4538.7508-1.289 1.9746-1.4652 2.2956l-.1183-1.3976-2.298 2.426c-.7894-1.499-2.2329-2.6046-3.9443-2.9305-.4297-.7652-2.1677-1.3156-3.954-1.3566-.0651-.0024-.1303-.0024-.198-.0024zm0 .7483c1.9142.0169 3.8888.5962 3.7294 1.0283l-1.3445 3.6184c-.099.2631-.2317.5117-.5118.5117H10.1304c-.2824 0-.4152-.2486-.5117-.5117L8.2693 5.1865c-.1593-.4297 1.8153-1.0114 3.7295-1.026zM7.7697 5.3144a.3416.3416 0 00.0168.0531L9.1311 8.986c.169.4538.4538.8472.9921.8472h3.7512c.5383 0 .8231-.3934.992-.8472l1.3446-3.6185c.0073-.0169.0121-.0362.017-.053 2.281.5165 3.9997 2.566 3.9997 4.9967v1.4193c0 2.8146-2.0228 5.1754-4.6877 5.7065-.1086-.2366-.2704-.4345-.5045-.5552-.2776-.1448-.5842-.1328-.8714-.0217-.3018.1158-.6035.2775-.886.4345-.1424.0772-.2848.1593-.4248.239a.9548.9548 0 00-.4731-.1256h-.7628a.955.955 0 00-.4732.1255c-.14-.082-.2824-.1617-.4248-.239-.2824-.1569-.5817-.3186-.8859-.4344-.2872-.111-.5938-.1207-.8714.0217-.2341.1207-.3959.321-.5045.5552-2.6625-.531-4.6878-2.8919-4.6878-5.704v-1.4218c0-2.4308 1.7163-4.4802 3.9999-4.9968zm-7.389.4031L1.788 8.2931l.2583-1.948 1.6221 1.8563a5.6213 5.6213 0 00-.4103 2.1073v1.4218c0 .0821.0024.1618.0048.2438L1.8894 9.657l-.2245 1.4797C1.0011 9.2201.6535 7.7742.3808 5.7175zm23.2337 0c-.2679 2.0567-.6155 3.5026-1.2793 5.4192l-.227-1.4797-1.3734 2.3173c.0024-.082.0048-.1617.0048-.2438v-1.4218a5.5759 5.5759 0 00-.4128-2.1073l1.6222-1.8562.2582 1.948zM8.2694 9.8308c-.5432 0-1.038.2197-1.3929.577a1.9644 1.9644 0 00-.577 1.3952c0 .543.2198 1.038.577 1.3952.3549.3573.8473.577 1.3928.577.5456 0 1.038-.2197 1.3953-.577a1.9645 1.9645 0 00.5769-1.3952c0-.5432-.2221-1.038-.577-1.3952a1.9645 1.9645 0 00-1.3952-.577zm7.4612 0c-.5455 0-1.038.2197-1.3952.577a1.9644 1.9644 0 00-.5769 1.3952c0 .543.2221 1.038.577 1.3952.3572.3573.8496.577 1.3951.577.5432 0 1.038-.2197 1.3929-.577a1.9645 1.9645 0 00.577-1.3952c0-.5432-.2198-1.038-.577-1.3952-.3549-.3573-.8473-.577-1.3929-.577zm-7.4613.2559c.4756 0 .9028.1907 1.2142.502.3114.3115.502.7411.502 1.2143 0 .473-.193.9028-.502 1.2142a1.707 1.707 0 01-1.2142.502 1.7112 1.7112 0 01-1.2141-.502 1.7113 1.7113 0 01-.5022-1.2142c0-.4732.1908-.9028.5022-1.2142a1.7112 1.7112 0 011.2141-.5021zm7.4613 0c.4732 0 .9029.1907 1.2142.502.3114.3115.5022.7411.5022 1.2143 0 .473-.1908.9028-.5022 1.2142a1.707 1.707 0 01-1.2142.502 1.707 1.707 0 01-1.2141-.502 1.7113 1.7113 0 01-.5021-1.2142c0-.4732.1931-.9028.502-1.2142a1.707 1.707 0 011.2142-.5021zm-7.4613.3935c-.7314 0-1.3228.5913-1.3228 1.3228 0 .7314.5914 1.3228 1.3228 1.3228.7314 0 1.3252-.5914 1.3252-1.3228 0-.7315-.5938-1.3228-1.3252-1.3228zm7.4613 0c-.7314 0-1.3227.5913-1.3227 1.3228 0 .7314.5913 1.3228 1.3227 1.3228.7314 0 1.3229-.5914 1.3229-1.3228 0-.7315-.5915-1.3228-1.3229-1.3228zm-9.023 4.328l-.2535.2125c.35.3983 1.3445 1.3035 3.167 1.3035H14.3813c1.82 0 2.817-.9052 3.167-1.3035l-.2534-.2125c-.309.3525-1.2166 1.1877-2.9136 1.1877H9.6211c-1.6994 0-2.6046-.8352-2.9136-1.1877zm2.6914 2.4773c.0766-.0012.161.0146.2535.0501.2897.111.6662.3138 1.12.572a.9495.9495 0 00-.1206.466v.0627c-.2149-.0676-.507-.14-.8811-.2269.6204.391.432.2897.8497.4973-.4635.2752-.2535.1376-.898.6228.3935-.1304.7024-.2366.9294-.3283a.9562.9562 0 00.1496.4997c-.3693.2196-.8159.449-1.2335.56-.8393.2244-1.182-2.76-.169-2.7754zm5.2015 0c1.0115.0154.6708 2.9998-.1685 2.7754-.4152-.111-.8642-.3404-1.236-.56a.9854.9854 0 00.1497-.4708c.2173.0845.5045.1835.857.2994-.6252-.4683-.449-.3573-.8546-.5963v-.0483c.3621-.1786.21-.099.8063-.4755-.3331.0773-.601.1449-.8063.2052v-.041a.9494.9494 0 00-.1206-.466c.4562-.2582.8303-.461 1.12-.572.0923-.0355.1765-.0513.253-.0501" /></ svg > } }