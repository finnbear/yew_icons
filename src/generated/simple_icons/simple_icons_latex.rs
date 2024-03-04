use crate :: IconProps ; # [inline (never)] pub fn simple_icons_latex (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M2.176 2.814c.233.42.476.78.73 1.09.247-.013 1.132.456 1.312.523.508.282 1.063.63 1.567.966.505.337.96.662 1.272.9.156.12.278.218.352.286a.483.483 0 01.078.082.08.08 0 01.01.021.06.06 0 01-.004.047.057.057 0 01-.04.03.077.077 0 01-.028 0c-.057 0-.203-.163-.497-.415a23.474 23.474 0 00-2.759-1.827c-.504-.28-.956-.542-1.264-.613a2.322 2.322 0 00-.36-.025 2.706 2.706 0 00-.788.133c.494.414.91.716 1.28.949-.57-.182-1.182-.21-1.902.133.526.329.967.567 1.354.745 1.103.156 2.258.696 3.224 1.309.483.307.904.615 1.219.867.157.125.29.237.39.328.098.091.174.154.197.21.03.073-.019.104-.084.058-.032-.022-.088-.102-.184-.191a7.35 7.35 0 00-.384-.327c-.312-.25-.729-.552-1.209-.857-.893-.562-2.232-1.013-3.173-1.397-.602-.11-1.225-.06-1.906.39.449.2.837.349 1.182.463.812 0 1.892.365 2.935.922 1.042.556 2.04 1.214 2.523 1.774.066.077-.016.126-.074.07-.52-.495-1.463-1.204-2.498-1.756-.639-.337-2.153-1.01-2.886-1.01l.004.002c-.567.02-1.13.195-1.679.716.477.118.885.196 1.244.249-.44.088-.87.3-1.289.722.324.07.616.122.882.162-.328.159-.639.404-.923.78.373.03.703.042 1 .044-.36.166-.696.43-.996.85.533.027.98.025 1.364.003-.422.172-.812.464-1.145.968.662.01 1.188-.022 1.628-.076l-.006.002c.99-.073 2.297.127 2.962.847.052.057-.024.118-.072.074-.648-.58-1.493-.827-2.89-.921h-.002c-.543.149-1.046.446-1.46 1.074.536.008.982-.013 1.366-.05-.469.257-.873.644-1.139 1.306.483-.092.888-.19 1.237-.292-.363.265-.668.636-.873 1.194.324-.072.612-.146.871-.221a2.519 2.519 0 00-.513 1.095c.352-.13.655-.254.926-.377-.257.3-.453.681-.55 1.19.495-.199.899-.388 1.238-.568-.31.333-.543.76-.635 1.356a11.816 11.816 0 001.442-.744c-.433.362-.764.843-.879 1.587.788-.348 1.339-.663 1.767-.955-.184.372-.282.806-.235 1.348.762-.584 1.243-1.056 1.602-1.473-.024.269-.003.56.077.884.546-.939 1.089-1.212 1.65-1.526-.895.451-.762.79-.762 1.184.683-.72 1.635-1.482 1.927-1.96-.39.585-.547 1.14-.65 1.63-1.993 1.054-3.207 1.329-4.568 1.75.528.194 1.093.383.859.652l-.624.622c.399-.124.805-.3 1.158-.059-.035.327-.447.492-.8.683.621-.224.756-.172.92-.12.081.393-.203.603-.388.862 1.565-1.19 3.606-2.13 5.044-2.522 2.022-.681 4.63-1.389 5.339-3.115l.712-2.847-.004.004c-.111-.034-.246-.063-.35-.133a.651.651 0 01-.235-.297c-.252.065-.44.03-.56-.088-.117-.117-.167-.296-.203-.491-.203.041-.362.016-.467-.077-.116-.101-.17-.26-.198-.444l-.008-.039.037-.015a.842.842 0 00.302-.194.257.257 0 00.07-.225l-.006-.037.03-.016c.163-.093.345-.169.428-.28a.274.274 0 00.053-.21.88.88 0 00-.155-.357l-.027-.04.04-.027c.118-.09.244-.179.308-.26.032-.04.048-.076.047-.11 0-.033-.015-.07-.064-.117l-.098-.094.135.006c.213.01.395-.007.538-.053a.504.504 0 00.274-.197c-.007-.033-.02-.063-.02-.098a.484.484 0 01.967 0c0 .044-.015.084-.026.125.177.014.347.01.507-.06l.002.001.035-.013c.236-.085.334.045.72-.456-1.69-2.19-4.157-.635-4.977 1.622-.21.576-1.405.578-1.751 0-1.37-2.95-5.53-6.068-9.07-7.218zm.86 2.145c.906.293 1.913.782 2.77 1.328.43.273.813.543 1.114.779.301.236.566.473.62.575.054.102 0 .14-.082.06-.081-.078-.303-.32-.6-.553-.298-.234-.68-.505-1.106-.777-.775-.49-1.982-.958-2.716-1.412zm-1.7 2.7c1.116.014 2.35.447 3.434.997.541.275 1.023.567 1.395.83.372.263.672.524.734.657.061.134-.02.13-.087.055a4.401 4.401 0 00-.704-.626 11.47 11.47 0 00-1.385-.826C3.76 8.264 2.439 7.82 1.336 7.66zm14.916.772a.381.381 0 100 .762.381.381 0 000-.762zM1.7 8.478c.822.072 1.72.368 2.534.75 1.086.509 2.035 1.158 2.434 1.667.035.045-.014.131-.08.062-.428-.44-1.322-1.131-2.397-1.635-.913-.421-2.282-.87-3.262-.78.251-.03.497-.088.771-.064zm16.339.01c-.366.475-.53.423-.703.464.094.43.35.586.585.77l-.06.012c2.315-.447 4.186-.286 6.139-.236l-5.961-1.01zm-.178 1.246h-.002l-.004.016.006-.016zm-.625-.757c-.183.074-.373.076-.563.059a.477.477 0 01-.42.26.483.483 0 01-.435-.278.609.609 0 01-.274.188c-.139.045-.308.057-.493.055.02.035.054.068.055.104a.273.273 0 01-.069.174c-.073.092-.189.17-.295.248.087.141.137.26.149.362a.39.39 0 01-.07.284c-.106.14-.288.21-.439.293a.374.374 0 01-.09.268.89.89 0 01-.297.198c.027.156.074.283.154.354.086.076.211.103.425.047l.055-.014.01.055c.033.207.088.385.187.483.1.099.244.135.503.055l.049-.015.016.048c.05.142.12.223.209.282.087.06.247.112.358.147.798-.869 1.525-1.772 1.884-2.86-.225-.177-.506-.338-.609-.797zm-16.23.386c1.165-.08 2.283.196 3.202.626.92.43 1.658.939 1.974 1.307.075.087-.019.12-.072.072a8.187 8.187 0 00-1.947-1.29c-.904-.414-2.193-.644-3.157-.715zm.864.802c.61.02 1.24.155 1.806.352.756.262 1.421.614 1.747.98.045.05-.007.127-.074.069-.349-.304-.961-.693-1.706-.951-.574-.195-1.613-.369-2.268-.397.197-.022.292-.06.495-.053zm1.05 1.788c.423.034.886.133 1.341.407.043.026.049.136-.049.09-.856-.402-1.326-.49-2.457-.31.386-.128.74-.221 1.164-.187zm-.04.788c.4-.035.784-.002 1.297.204.044.018.08.126-.033.094-.857-.243-1.167-.328-2.287.104.28-.229.622-.366 1.023-.402zm1.285.687c.317-.023.635-.026.934.006.052.006.055.105-.006.102a7.87 7.87 0 00-1.837.115c-.243.046-.423.043-1.405.458.287-.233.794-.452 1.385-.56a8.91 8.91 0 01.93-.12zm1.28.49c.099.003.062.104.006.103-.728-.01-1.304.132-1.875.295a9.78 9.78 0 00-1.318.525c.283-.23.713-.457 1.291-.622.579-.166 1.248-.326 1.896-.302zm.528.398c.036-.005.105.084.018.1-.73.137-1.244.267-1.794.454-.216.074-.58.207-1.243.587.26-.269.656-.492 1.213-.68.558-.19 1.196-.37 1.806-.46zm.311.507c.075-.012.097.087.02.102-1.217.241-1.76.556-2.54 1.144.504-.523 1.297-1.051 2.52-1.246zm.595.448c.087-.013.11.087.021.1-.872.13-1.477.553-2.255 1.33.295-.493 1.004-1.24 2.234-1.43zm.372.39c.046-.006.114.073.023.1a2.634 2.634 0 00-.669.3c-.182.118-.3.2-.597.507.111-.245.296-.434.542-.59.247-.157.509-.293.7-.317z" /></ svg > } }