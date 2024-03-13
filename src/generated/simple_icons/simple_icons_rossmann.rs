use crate :: IconProps ; # [inline (never)] pub fn simple_icons_rossmann (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M20.5326 10.514c0-.032-.016-.048-.048-.048h-.2076c-.032 0-.048.016-.048.048v2.0453c0 .0479-.016.0639-.048.032-.1118-.112-1.9014-1.9974-1.9973-2.0933-.032-.032-.0479-.032-.0639-.032H17.96c-.032 0-.0479.016-.0479.048v2.924c0 .032.016.048.048.048h.2077c.032 0 .0479-.016.0479-.048V11.361c0-.08.048-.032.064-.016.1277.1438 1.8055 1.9014 1.9973 2.1092.032.032.0479.032.0639.032h.1278c.032 0 .048-.016.048-.048.016.016.016-2.9241.016-2.9241zm-16.3781-.1758c-.9108 0-1.6618.735-1.6618 1.6618 0 .9108.735 1.6618 1.6618 1.6618.9267 0 1.6617-.735 1.6617-1.6618s-.751-1.6618-1.6617-1.6618zm0 .2876c.4154 0 .783.1758 1.0226.4634h-.4794l.048-.0959c.048-.1118-.1438-.1917-.1918-.0798l-.0799.1917H3.1318a1.3312 1.3312 0 0 1 1.0227-.4794ZM2.7963 12c0-.2557.0799-.5113.2077-.719 0 0 1.2623.0479 1.2783.0479.016 0 .032 0 .048.032.016.016.1278.2876.1278.3036 0 .016 0 .0479-.048.0479H3.1798c-.08 0-.1598.0639-.1598.1438v.4634c0 .032.016.0639.032.0959.0319.0479.0958.0639.0958.0639.016 0 .016 0 .016-.016v-.5912c0-.016.016-.016.016-.016h.1278s.016 0 0 .016c-.048.0639-.064.1278-.064.2237 0 .1598.1279.2876.1599.3036 0 0 .016.016 0 .032 0 .016-.1279.2396-.1279.2396 0 .016-.016.032 0 .048l.5912.5912c-.5912-.096-1.0705-.6552-1.0705-1.3103Zm1.422 1.3582s-.5752-.5752-.6071-.6232c-.016-.016 0-.032 0-.032 0-.016.2077-.3994.2237-.4154.016-.016.032-.016.032-.016h.1278s.016 0 .016.016c-.016.016-.1918.3675-.1918.3835v.032c0 .016.6232.6231.6232.6231-.064.016-.1438.032-.2237.032zm.4475-.0959-.5433-.5433c-.016-.016 0-.032 0-.032l.1917-.3834c.016-.016.016-.016.032-.016h.3356c.016 0 .016.016.016.016l-.096.2077v.032l.1279.0799c.016.016.032 0 .032-.016l.1278-.3036c.016-.016.016-.016.032-.016h.1118s.016 0 .016.016-.0959.2237-.0959.2237v.032l.1278.0798c.016.016.032 0 .032-.016s.1438-.3195.1438-.3355c.032-.0799.016-.2237-.1438-.2237h-.1918c-.016 0-.032-.016-.032-.016v-.6391c0-.032.016-.032.032-.048.016 0 .3516-.0479.3516-.0479.1278.2077.2077.4474.2077.719.032.5273-.3196 1.0227-.815 1.2304zm2.1571-2.173a.432.432 0 0 1 .3675-.4315c.2077-.048.4634 0 .6072.1758.016.016.032.0479.048.0798.016.016.0319.032.0639.048h.0798c.016 0 .048-.016.048-.048 0-.0639-.016-.2716-.016-.3035 0-.048-.016-.048-.048-.064a1.3263 1.3263 0 0 0-.3355-.0958c-.2716-.048-.5592-.032-.8149.0799-.2397.0958-.4154.2876-.4794.5432-.0479.1918-.032.3995.064.5753.0958.1598.2396.2556.4154.3355.1758.08.3515.1438.5113.2077.2077.096.3835.1918.4314.3676.064.2237.032.4474-.1438.6071-.2396.2078-.8788.1758-1.1025-.1438-.032-.0639-.048-.0958-.1118-.0958-.032 0-.096 0-.096.048v.3355c0 .032 0 .0639.048.0798.064.032.1438.048.2077.08.1119.0319.2397.0479.3516.0638.783.08 1.3422-.3195 1.3422-.9906 0-.2397-.0959-.4794-.2876-.6232-.1758-.1278-.3835-.2077-.5912-.2716-.1119-.032-.2557-.08-.3516-.1438-.1438-.096-.2077-.2397-.2077-.4155m-4.458 2.3329c-.048-.064-.9268-1.1824-.9748-1.2463-.016-.032-.016-.048.048-.064.1278-.0479.5752-.2556.5912-.7829.016-.4474-.2876-.8469-.783-.8469H.048c-.032-.016-.0479 0-.0479.032v2.924c0 .032.016.048.048.048h.3674c.032 0 .048-.016.048-.048v-1.2303c0-.032.016-.048.048-.048H.735c.016 0 .048 0 .08.048.0319.032.9746 1.2623.9746 1.2623.016.032.048.032.064.032h.4473c.064 0 .08-.048.064-.0799zM.719 11.8881H.5113c-.016 0-.032-.032-.032-.0639v-1.0546c0-.032.016-.0479.048-.0479h.3196c.3355 0 .6551.0959.6551.5273 0 .2397-.1119.6392-.783.6392zM24 10.514c0-.032-.016-.048-.048-.048h-.2077c-.032 0-.0479.016-.0479.048v2.0453c0 .0479-.016.0639-.048.032-.1118-.112-1.9014-1.9974-1.9973-2.0933-.032-.032-.048-.032-.0639-.032h-.1598c-.032 0-.048.016-.048.048v2.924c0 .032.016.048.048.048h.2077c.032 0 .048-.016.048-.048V11.361c0-.08.048-.032.0639-.016.1278.1438 1.8056 1.9014 1.9973 2.1092.032.032.048.032.064.032h.1278c.032 0 .048-.016.048-.048C24 13.454 24 10.514 24 10.514Zm-14.8282.5752a.432.432 0 0 1 .3675-.4314c.2077-.048.4634 0 .6072.1758.016.016.032.0479.048.0798.0159.016.0319.032.0638.048h.064c.016 0 .0479-.016.0479-.048 0-.0639-.016-.2716-.016-.3035 0-.048-.016-.048-.048-.064-.0798-.0479-.1917-.0798-.3195-.1118-.2717-.048-.5593-.032-.815.0799-.2396.0958-.4154.2876-.4793.5433-.048.1917-.032.3994.064.5752.0958.1598.2396.2556.4154.3355.1757.08.3515.1438.5113.2078.2077.0958.3835.1917.4314.3675.064.2237.032.4474-.1438.6072-.2397.2077-.8788.1757-1.1025-.1438-.032-.064-.048-.096-.1119-.096-.032 0-.0958 0-.0958.048v.3356c0 .032 0 .0639.0479.0799.0639.032.1438.048.2077.0799.1119.032.2397.0479.3515.0639.783.0799 1.3423-.3196 1.3423-.9907 0-.2397-.096-.4794-.2877-.6232-.1757-.1278-.3835-.2077-.5912-.2716-.1118-.032-.2556-.0799-.3515-.1438-.1438-.08-.2077-.2237-.2077-.3995m7.2383 1.1345c.016.016 0 .048-.032.048h-.8468c-.032 0-.048-.016-.032-.048s.4155-.9587.4315-.9907c.016-.032.032-.032.0479 0 0 .032.4154.9747.4314.9907zm-.2237-1.6618c-.032-.0799-.0799-.0959-.1118-.0959h-.1279c-.016 0-.032 0-.0479.032-.016.032-1.2463 2.8922-1.2623 2.94-.016.032-.016.064.032.064h.2236c.016 0 .032 0 .048-.048.016-.0479.3515-.8149.3675-.8468.016-.032.048-.032.0799-.032h1.1185c.016 0 .048 0 .064.032 0 0 .3674.8309.3674.8469.016.032.032.0479.064.0479h.4154c.032 0 .048-.032.032-.064 0-.0159-1.2304-2.8122-1.2624-2.876zm-2.221-.0639c0-.032-.016-.032-.048-.032h-.1597c-.016 0-.032 0-.032.016-.016.016-.9268 2.1731-.9428 2.205-.016.032-.032.032-.0479 0 0-.0159-.9907-2.173-1.0066-2.189-.016-.016-.016-.032-.048-.032h-.1598c-.016 0-.032 0-.032.032 0 .016-.4633 2.956-.4633 2.956 0 .016 0 .048.032.048h.2077c.032 0 .0479-.016.0639-.048 0-.0319.3036-1.9813.3195-1.9973 0-.032.016-.032.032 0 .016.016.9268 2.0133.9268 2.0293 0 .016.016.016.032.016h.0958c.016 0 .016 0 .032-.016 0-.016.8468-1.9813.8628-2.0133.016-.032.016-.032.032 0 0 .016.3515 1.9814.3515 1.9973 0 .032.016.032.048.032h.3994c.032 0 .048-.032.048-.048-.032-.0798-.5433-2.94-.5433-2.956z" /></ svg > } }