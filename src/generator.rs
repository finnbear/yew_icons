use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use regex::Regex;
use std::fs::{read, read_dir, write};
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;

fn main() {
    let mut variants = Vec::new();
    let mut functions = Vec::new();
    let mut cases = Vec::new();

    let width_regex = Regex::new(r##"width="[0-9]*""##).unwrap();
    let height_regex = Regex::new(r##"height="[0-9]*""##).unwrap();
    let comment_regex = Regex::new(r##"<!--(.*?)-->"##).unwrap();

    let mut generate = |prefix: &str, dir: &str| {
        let feature = prefix.to_case(Case::Snake);

        let result = read_dir(dir);
        let mut paths: Vec<_> = result
            .unwrap()
            .map(|result| result.unwrap())
            .map(|entry| entry.path())
            .map(|path_buf| path_buf.to_str().unwrap().to_owned())
            .collect();

        paths.sort();

        for path in paths {
            let file_name = path.split('/').last().unwrap();
            if !file_name.ends_with(".svg") {
                panic!("never happens?");
            }
            let name = prefix.to_owned() + "-" + file_name.split('.').next().unwrap();

            let variant = to_ident(&name.to_case(Case::UpperCamel));
            variants.push(quote! {
                #[cfg(feature = #feature)]
                #variant
            });

            let contents = read(&path).expect(&path);
            let svg = std::str::from_utf8(&contents).unwrap();

            let svg = width_regex.replace(svg, "").into_owned();
            let svg = height_regex.replace(&svg, "").into_owned();
            let svg = comment_regex.replace(&svg, "").into_owned();

            let mut replacement =
                r#"xmlns="http://www.w3.org/2000/svg" {width} {height} onclick={on_click}"#
                    .to_string();

            if !svg.contains("stroke") {
                replacement += r#" stroke="currentColor" fill="currentColor""#;
            }

            let svg = svg.replace(r#"xmlns="http://www.w3.org/2000/svg""#, &replacement);

            let svg_tokens = TokenStream::from_str(&svg).unwrap();

            let function_name = to_ident(&name.to_case(Case::Snake));

            functions.push(quote! {
                #[cfg(feature = #feature)]
                fn #function_name(width: String, height: String, on_click: Option<Callback<MouseEvent>>) -> Html {
                    yew::html! {
                        #svg_tokens
                    }
                }
            });

            cases.push(quote! {
                #[cfg(feature = #feature)]
                IconId::#variant => #function_name(width, height, on_click)
            });
        }
    };

    generate("Feather", "feather/icons");
    generate("FontAwesomeRegular", "Font-Awesome/svgs/regular");
    generate("FontAwesomeSolid", "Font-Awesome/svgs/solid");
    generate("Octicons", "octicons/icons");

    let tokens = quote! {
        use yew::prelude::*;

        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        #[cfg_attr(feature = "iterate_icon_id", derive(enum_iterator::IntoEnumIterator))]
        #[non_exhaustive]
        pub enum IconId {
            #(#variants),*
        }

        #(#functions)*

        pub fn get_svg(icon_id: IconId, width: String, height: String, on_click: Option<Callback<MouseEvent>>) -> Html {
            match icon_id {
                #(#cases),*
            }
        }
    };

    let output = reformat(tokens.to_string()).unwrap();

    write("src/generated.rs", output).unwrap();
}

fn to_ident(string: &str) -> Ident {
    Ident::new(string, Span::call_site())
}

// https://github.com/rust-analyzer/rust-analyzer/blob/ada9e16537c22b490d13cdd54b9e1e4885856a4c/xtask/src/codegen.rs#L66-L78
fn reformat(text: impl std::fmt::Display) -> Result<String, String> {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    write!(rustfmt.stdin.take().unwrap(), "{}", text).map_err(|e| e.to_string())?;
    let output = rustfmt.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let preamble = "Generated file, do not edit by hand, see `src/generator.rs`";
    Ok(format!("//! {}\n\n{}", preamble, stdout))
}
