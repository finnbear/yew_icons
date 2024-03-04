use crate :: IconProps ; # [inline (never)] pub fn simple_icons_esphome (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style , role } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } role = { role . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M7.253 2.755c-.676 0-1.231.555-1.231 1.232v.976h-.083a.722.722 0 00-.717.716v11.682H.71v-.57h3.544a.355.355 0 00.354-.354v-1.279a.355.355 0 00-.354-.355H.709v-.565h3.544a.355.355 0 00.354-.355v-1.278a.355.355 0 00-.354-.355H.709v-.569h3.544a.355.355 0 00.354-.355V10.05a.355.355 0 00-.354-.354H.709V6.113a.355.355 0 00-.355-.355.355.355 0 00-.354.355v3.937a.355.355 0 00.354.355h3.544v.566H.354a.355.355 0 00-.354.355v1.279a.355.355 0 00.354.354h3.544v.57H.354a.355.355 0 00-.354.354v1.275a.355.355 0 00.354.355h3.544v.57H.354a.355.355 0 00-.354.354v1.278a.355.355 0 00.354.355h4.868v.086c0 .389.323.716.717.716h.083v1.14c0 .677.555 1.233 1.231 1.233.677 0 1.233-.556 1.233-1.232v-1.14h.477v1.137c0 .676.556 1.232 1.232 1.232.677 0 1.232-.556 1.232-1.232v-1.138h.481v1.138c0 .676.556 1.232 1.232 1.232.676 0 1.233-.556 1.233-1.232v-1.138h.48v1.138c0 .676.556 1.232 1.232 1.232.677 0 1.232-.556 1.232-1.232v-1.138h.481v1.138c0 .676.556 1.232 1.232 1.232.676 0 1.233-.556 1.233-1.232v-1.138h.477v1.138c0 .676.555 1.232 1.231 1.232.677 0 1.233-.556 1.233-1.232v-1.138h.079c.39 0 .717-.323.717-.716V5.679a.723.723 0 00-.714-.716h-.082v-.979c0-.676-.556-1.231-1.232-1.23h-.001c-.676.001-1.231.557-1.231 1.233v.976h-.477v-.98c0-.675-.557-1.23-1.233-1.228h-.001c-.676 0-1.23.556-1.23 1.232v.976h-.482v-.976c0-.677-.555-1.232-1.232-1.232-.676 0-1.232.555-1.232 1.232v.976h-.48v-.976c0-.677-.557-1.232-1.233-1.232s-1.232.555-1.232 1.232v.976h-.48v-.976c0-.677-.556-1.232-1.233-1.232-.676 0-1.232.555-1.232 1.232v.976h-.477v-.976c0-.677-.556-1.232-1.233-1.232zm0 .715a.51.51 0 01.517.517v.976H6.737v-.976a.51.51 0 01.516-.517zm2.942 0a.51.51 0 01.517.517v.976H9.679v-.976a.51.51 0 01.516-.517zm2.945 0a.51.51 0 01.516.517v.976h-1.032v-.976a.51.51 0 01.516-.517zm2.945 0a.51.51 0 01.517.517v.976h-1.033v-.976a.51.51 0 01.516-.517zm2.945 0h.001a.507.507 0 01.515.513v.98h-1.032v-.976a.51.51 0 01.516-.517zm2.942.001h.001a.507.507 0 01.515.513v.979h-1.032v-.976a.51.51 0 01.516-.516zM6.018 5.758h17.186v12.319H6.018zm8.63 2.777a.322.322 0 00-.234.095l-3.776 3.78a.322.322 0 00.228.55h.62v2.225a.322.322 0 00.323.322h5.67a.322.322 0 00.322-.322V12.96h.621a.322.322 0 00.228-.55l-.856-.859v-1.533a.322.322 0 00-.322-.323h-.591a.322.322 0 00-.323.323v.3L14.87 8.63a.322.322 0 00-.221-.095zm-7.91 10.337H7.77v1.14a.51.51 0 01-.517.517.51.51 0 01-.516-.516zm2.94 0h1.034v1.138a.51.51 0 01-.517.516.51.51 0 01-.516-.516zm2.946 0h1.032v1.138a.51.51 0 01-.516.516.51.51 0 01-.516-.516zm2.945 0h1.033v1.138a.51.51 0 01-.517.516.51.51 0 01-.516-.516zm2.945 0h1.032v1.138a.51.51 0 01-.516.516.51.51 0 01-.516-.516zm2.941 0h1.033v1.138a.51.51 0 01-.517.516.51.51 0 01-.516-.516z" /></ svg > } }