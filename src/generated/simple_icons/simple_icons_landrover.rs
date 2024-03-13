use crate :: IconProps ; # [inline (never)] pub fn simple_icons_landrover (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M10.182 11.491h.717l.625-1.85.365 1.85h.718l.978-2.858-.75.001-.522 1.554-.299-1.554h-1.01l-.025.074.05.048c.053.053.05.114.005.246l-.852 2.489zm3.749-2.489l-.853 2.49h1.259c.54 0 .847-.12 1.098-.387.313-.332.6-1.195.59-1.67-.009-.485-.322-.8-.98-.8H13.9l-.024.073.05.048c.052.053.05.114.004.246m.685.31h.313c.213 0 .33.071.333.236.002.106-.027.252-.137.565-.098.28-.176.443-.287.562-.088.094-.207.135-.478.135h-.261l.517-1.498zm4.066.347H16.78l-.247.68h1.318l-1.497.509-.093.243 2.157-.71.264-.722zM5.043 11.492h1.95l.236-.682H6.095l.75-2.176h-.979l-.024.074.049.048c.053.053.051.114.005.246l-.853 2.49zm2.153-.001h.83l.359-.632h.859l.008.632h.753l-.162-2.857H8.716l-.025.074.036.036c.06.058.035.142-.184.49l-1.347 2.257zm1.996-2.067h.026l.014.864h-.527l.487-.864zM8.01 12.11H6.776l-.025.074.05.05c.053.051.05.113.004.245l-.862 2.49h.788l.337-.989h.267l.296.988h.792l-.372-1.01a.76.76 0 0 0 .433-.194c.149-.136.252-.326.337-.607.124-.407.049-.64-.031-.76-.11-.164-.342-.287-.78-.287m.068 1.038c-.05.122-.104.151-.175.18a.845.845 0 0 1-.245.026H7.28l.205-.591h.455c.136 0 .192.07.194.152a.759.759 0 0 1-.057.233m10.436-1.038H17.28l-.024.074.05.05c.053.051.05.113.005.245l-.863 2.49h.789l.336-.988h.267l.297.987h.79l-.37-1.01a.758.758 0 0 0 .431-.194c.15-.136.253-.326.338-.607.124-.407.05-.64-.03-.76-.11-.164-.343-.287-.782-.287m.069 1.038c-.05.122-.104.151-.174.18a.849.849 0 0 1-.245.026h-.376l.203-.591h.455c.136 0 .192.07.193.152a.747.747 0 0 1-.056.233m1.367-4.742c-2.365-1.222-5.31-1.69-7.95-1.69s-5.585.468-7.95 1.69C2.72 9.093 1.035 10.331 1.035 12c0 1.669 1.686 2.907 3.015 3.594 2.365 1.222 5.31 1.69 7.95 1.69 2.64 0 5.585-.467 7.95-1.69 1.329-.687 3.015-1.925 3.015-3.594 0-1.67-1.685-2.907-3.015-3.594m-.12 6.954c-2.33 1.203-5.227 1.66-7.83 1.66-2.603 0-5.5-.457-7.83-1.66C2.939 14.721 1.3 13.546 1.3 12c0-1.548 1.638-2.722 2.87-3.359C6.5 7.438 9.398 6.981 12 6.981c2.603 0 5.5.456 7.83 1.66 1.233.637 2.87 1.81 2.87 3.36 0 1.547-1.638 2.721-2.87 3.359m.595-7.873C18.171 6.322 15.18 5.681 12 5.681c-3.179 0-6.17.641-8.425 1.806C2.24 8.177 0 9.681 0 12s2.24 3.823 3.575 4.513C5.829 17.678 8.821 18.32 12 18.32c3.18 0 6.171-.641 8.425-1.806C21.76 15.823 24 14.32 24 12c0-2.32-2.24-3.823-3.575-4.513m-.293 8.46C17.967 17.064 15.08 17.68 12 17.68s-5.966-.616-8.132-1.735C1.786 14.87.638 13.468.638 12c0-1.469 1.148-2.87 3.23-3.946C6.034 6.936 8.921 6.32 12 6.32c3.08 0 5.967.617 8.132 1.735 2.083 1.075 3.23 2.477 3.23 3.946 0 1.468-1.147 2.87-3.23 3.946m-9.41-3.87h-.091c-.539 0-.846.144-1.099.412-.314.333-.609 1.23-.603 1.706.006.484.32.82.978.82h.096c.54 0 .848-.145 1.1-.413.314-.333.603-1.23.597-1.706-.006-.484-.32-.82-.979-.82m.046 1.515c-.098.281-.175.462-.288.581-.09.094-.207.161-.48.161-.212 0-.33-.087-.33-.251-.002-.107.027-.268.141-.582.098-.28.175-.462.289-.581.088-.094.207-.161.478-.161.213 0 .331.087.33.25.003.108-.026.27-.14.583M6.035 12.48l-2.157.709-.263.723h1.902l.248-.681H4.446l1.497-.507.092-.244zm7.846-.369l-1.046 2.055h-.022l-.053-2.055h-.875l-.034.074.075.046c.073.051.093.103.102.246l.119 2.492h.975l1.573-2.858h-.814zm2.396 1.712l.219-.597H15.49l.156-.457h1.088l.24-.66H14.92l-.029.084.05.049c.053.051.05.112.004.246l-.855 2.48h1.92l.232-.663h-1.12l.164-.482h.991z" /></ svg > } }