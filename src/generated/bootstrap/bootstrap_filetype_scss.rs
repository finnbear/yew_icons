use crate :: IconProps ; # [inline (never)] pub fn bootstrap_filetype_scss (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/twbs/icons - Licensed under MIT" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" viewBox = "0 0 16 16" > if let Some (title) = title . clone () { < title > { title } </ title > } < path fill - rule = "evenodd" d = "M14 4.5V11h-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5L14 4.5ZM1.356 15.29a1.176 1.176 0 0 1-.111-.449h.765a.578.578 0 0 0 .255.384c.07.049.153.087.249.114.095.028.202.041.319.041.164 0 .302-.023.413-.07a.559.559 0 0 0 .255-.193.506.506 0 0 0 .085-.29.387.387 0 0 0-.153-.326c-.101-.08-.255-.144-.462-.193l-.619-.143a1.72 1.72 0 0 1-.539-.214 1.001 1.001 0 0 1-.351-.367 1.068 1.068 0 0 1-.123-.524c0-.244.063-.457.19-.639.127-.181.303-.322.528-.422.224-.1.483-.149.776-.149.305 0 .564.05.78.152.216.102.383.239.5.41.12.17.186.359.2.566h-.75a.56.56 0 0 0-.12-.258.624.624 0 0 0-.247-.181.923.923 0 0 0-.369-.068c-.217 0-.388.05-.513.152a.472.472 0 0 0-.184.384c0 .121.048.22.143.3a.97.97 0 0 0 .405.175l.62.143c.217.05.406.12.566.211.16.09.285.21.375.358.09.148.135.335.135.56 0 .247-.063.466-.188.656a1.216 1.216 0 0 1-.539.439c-.234.105-.52.158-.858.158-.254 0-.476-.03-.665-.09a1.404 1.404 0 0 1-.478-.252 1.13 1.13 0 0 1-.29-.375Zm4.274-2.23a1.732 1.732 0 0 0-.103.633v.495c0 .246.034.455.103.627a.833.833 0 0 0 .298.392.846.846 0 0 0 .478.132.868.868 0 0 0 .401-.088.7.7 0 0 0 .273-.249.798.798 0 0 0 .117-.363h.765v.076a1.27 1.27 0 0 1-.226.674 1.39 1.39 0 0 1-.55.454 1.81 1.81 0 0 1-.786.164c-.36 0-.664-.072-.914-.217a1.424 1.424 0 0 1-.571-.626c-.13-.272-.194-.597-.194-.976v-.498c0-.38.066-.705.197-.979a1.44 1.44 0 0 1 .57-.633c.253-.148.557-.222.912-.222.219 0 .421.032.607.097.187.062.35.153.489.272a1.324 1.324 0 0 1 .466.964v.073h-.765a.85.85 0 0 0-.12-.38.7.7 0 0 0-.273-.261.803.803 0 0 0-.398-.097.814.814 0 0 0-.475.138.868.868 0 0 0-.301.398Zm2.609 1.781a1.13 1.13 0 0 0 .401.823c.129.108.288.192.478.252.19.061.41.091.665.091.338 0 .624-.053.858-.158.236-.105.416-.252.54-.44a1.17 1.17 0 0 0 .187-.656c0-.224-.045-.41-.135-.56a1.002 1.002 0 0 0-.375-.357 2.028 2.028 0 0 0-.566-.21l-.62-.144a.97.97 0 0 1-.405-.176.37.37 0 0 1-.143-.299c0-.156.061-.284.184-.384.125-.101.296-.152.513-.152.142 0 .265.023.369.068a.623.623 0 0 1 .246.181.56.56 0 0 1 .12.258h.75a1.091 1.091 0 0 0-.2-.566 1.21 1.21 0 0 0-.5-.41 1.813 1.813 0 0 0-.78-.152c-.292 0-.551.05-.776.15-.224.099-.4.24-.527.421-.127.182-.19.395-.19.639 0 .201.04.376.123.524.082.149.199.27.351.367.152.095.332.167.54.213l.617.144c.207.049.362.113.463.193a.387.387 0 0 1 .153.326.512.512 0 0 1-.085.29.558.558 0 0 1-.255.193 1.07 1.07 0 0 1-.413.07c-.118 0-.224-.013-.32-.04a.837.837 0 0 1-.249-.115.578.578 0 0 1-.255-.384H8.24Zm3.502.449a1.176 1.176 0 0 1-.11-.449h.764a.578.578 0 0 0 .255.384c.07.049.153.087.249.114.095.028.202.041.319.041.164 0 .302-.023.413-.07a.558.558 0 0 0 .255-.193.506.506 0 0 0 .085-.29.387.387 0 0 0-.152-.326c-.102-.08-.256-.144-.463-.193l-.618-.143a1.72 1.72 0 0 1-.54-.214 1.002 1.002 0 0 1-.351-.367 1.068 1.068 0 0 1-.123-.524c0-.244.063-.457.19-.639.127-.181.303-.322.528-.422.224-.1.483-.149.776-.149.305 0 .565.05.78.152.216.102.383.239.5.41.12.17.186.359.2.566h-.75a.56.56 0 0 0-.12-.258.623.623 0 0 0-.247-.181.923.923 0 0 0-.369-.068c-.217 0-.387.05-.512.152a.472.472 0 0 0-.185.384c0 .121.048.22.143.3a.97.97 0 0 0 .405.175l.62.143c.218.05.406.12.566.211.16.09.285.21.375.358.09.148.135.335.135.56 0 .247-.062.466-.187.656a1.217 1.217 0 0 1-.54.439c-.234.105-.52.158-.858.158-.254 0-.476-.03-.665-.09a1.404 1.404 0 0 1-.478-.252 1.131 1.131 0 0 1-.29-.375Z" /> </ svg > } }