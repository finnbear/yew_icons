use crate :: IconProps ; # [inline (never)] pub fn simple_icons_battledotnet (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M20.001 3.106c0-.059 0-.187.063-.187.09 0 .167.19.196.27.486 1.548.35 3.449-.345 5.588 2.756 1.473 4.338 3.382 4.052 5.168-.38 2.347-3.824 3.727-8.462 3.565a27.12 27.12 0 001.55-1.825c1.786-.347 3.066-1.053 3.4-2.184.246-.823-.32-1.736-1.396-2.594-1.768 3.723-4.722 7.058-7.451 8.985 1.323 1.687 2.556 2.858 3.964 3.516.052.025.164.09.126.148-.044.078-.252.05-.33.034-1.585-.352-3.158-1.42-4.673-3.093-2.652 1.65-5.099 2.066-6.502.925-1.842-1.5-1.316-5.178 1.137-9.106.193.631.505 1.526.806 2.253-.594 1.721-.622 3.183.19 4.041.59.623 1.664.59 2.945.088-2.325-3.395-3.738-7.62-4.04-10.954-2.124.303-3.754.785-5.027 1.675-.048.033-.16.098-.195.04-.044-.078.082-.24.137-.304 1.098-1.196 2.812-2.028 5.012-2.496-.102-3.124.758-5.45 2.45-6.093 2.22-.846 5.14 1.448 7.316 5.542a26.617 26.617 0 00-2.354-.429c-1.193-1.372-2.446-2.13-3.594-1.854-.834.2-1.341 1.146-1.547 2.502 4.108-.332 8.473.56 11.51 1.964.8-1.984 1.197-3.637 1.062-5.185zm-9.598 15.057c3.252-1.859 5.889-4.696 7.638-7.965-3.24-1.88-7.013-2.75-10.716-2.627-.008 3.745 1.124 7.451 3.082 10.594Z" /></ svg > } }