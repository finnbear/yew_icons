use crate :: IconProps ; # [inline (never)] pub fn simple_icons_macpaw (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M9.622.21c.235-.126 1.12-.432 1.38.06.26.492-.483 1.061-.736 1.196-.252.136-.664.122-.898-.321A.675.675 0 0 1 9.622.21zm2.634 4.822c.236-.126 1.12-.431 1.38.06.26.492-.482 1.061-.735 1.196-.253.136-.664.123-.899-.32a.675.675 0 0 1 .254-.936ZM12 24a11.403 11.403 0 0 1-8.08-3.345 11.38 11.38 0 0 1-3.35-8.077 11.378 11.378 0 0 1 3.35-8.077 11.392 11.392 0 0 1 3.632-2.446 2.835 2.835 0 0 1 3.719 1.508 2.84 2.84 0 0 1-1.508 3.716c-.684.289-1.3.704-1.83 1.233a5.71 5.71 0 0 0-1.684 4.067 5.705 5.705 0 0 0 1.684 4.065A5.727 5.727 0 0 0 12 18.327a5.727 5.727 0 0 0 4.068-1.683 5.712 5.712 0 0 0 1.685-4.065 2.84 2.84 0 0 1 2.838-2.837 2.84 2.84 0 0 1 2.838 2.836 11.34 11.34 0 0 1-.9 4.447 11.367 11.367 0 0 1-2.447 3.63A11.422 11.422 0 0 1 12 24Zm.426-21.111c.263-.14 1.346-.533 1.635.016.29.549-.633 1.235-.915 1.386-.282.15-.742.136-1.003-.359a.754.754 0 0 1 .283-1.043Zm-1.004-1.806c.263-.14 1.32-.518 1.61.03.29.55-.608 1.221-.89 1.372-.282.151-.741.136-1.003-.358a.754.754 0 0 1 .283-1.044Z" /></ svg > } }