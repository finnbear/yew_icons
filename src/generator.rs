use convert_case::{Case, Casing};
use indicatif::{ProgressBar, ProgressStyle};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::fs::{create_dir_all, read, read_dir, remove_dir_all, write};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

fn main() {
    let mut features = Vec::new();
    let mut imports = Vec::new();
    let mut enumerate: Vec<_> = Vec::new();

    let width_regex = Regex::new(r##"[^-]width="[0-9a-z]*""##).unwrap();
    let height_regex = Regex::new(r##"[^-]height="[0-9a-z]*""##).unwrap();
    let role_regex = Regex::new(r##"[^-]role="[a-z]*""##).unwrap();
    let class_regex = Regex::new(r##"class="[A-Za-z0-9-_ ]*""##).unwrap();
    let text_regex = Regex::new(r##">(\s*[a-zA-Z.\-]+[a-zA-Z. \-]*)<"##).unwrap();
    let title_regex = Regex::new(r##"<title>.+</title>"##).unwrap();
    let desc_regex = Regex::new(r##"<desc>.+</desc>"##).unwrap();
    let comment_regex = Regex::new(r##"<!--(.*?)-->"##).unwrap();
    let clip_path_regex = Regex::new(r##"clip-path="url\(#.+\)" "##).unwrap();

    remove_dir_all("src/generated").unwrap();
    create_dir_all("src/generated").unwrap();

    let mb = indicatif::MultiProgress::new();

    let mut generate = |prefix: &str, dir: &str, license: &str| {
        let feature_name = prefix.to_case(Case::Snake);

        let feature_ident = to_ident(&feature_name);
        let mut icon_data = Vec::new();

        let result = read_dir(dir);
        let mut paths: Vec<_> = result
            .expect(dir)
            .map(|result| result.unwrap())
            .map(|entry| entry.path())
            .map(|path_buf| path_buf.to_str().unwrap().to_owned())
            .collect();

        paths.sort();

        let bar = ProgressBar::new(paths.len() as u64)
            .with_message(feature_name.clone())
            .with_style(ProgressStyle::with_template("{msg} {wide_bar} {pos}/{len}").unwrap());
        let bar = mb.add(bar);

        let items = paths.into_par_iter()
            .map(|path| {
                let file_name = path.split('/').last().unwrap();
                if !file_name.ends_with(".svg") {
                    panic!("never happens?");
                }
                let icon_name = file_name.split('.').next().unwrap();

                // Would like to use collection::NAME but NAME might start with a number.
                let name = prefix.to_owned() + "-" + icon_name;

                let contents = read(&path).expect(&path);
                let svg = std::str::from_utf8(&contents).unwrap();

                let svg = class_regex.replace_all(&svg, "");

                assert!(!svg.contains(r#"title="#), "already had title: {}", path);
                assert!(
                    !svg.contains(r#"class="#),
                    "already had class despite regex: {}",
                    path
                );

                // Ids not supported in HTML context.
                let svg = clip_path_regex.replace_all(&svg, " ").into_owned();

                // https://github.com/yammadev/flag-icons/blob/bd4bcf4f4829002cd10416029e05ba89a7554af4/svg/AE.svg
                let svg = svg.replace(r##"<?xml version="1.0" encoding="UTF-8"?>"##, "");

                // https://github.com/yammadev/flag-icons/blob/bd4bcf4f4829002cd10416029e05ba89a7554af4/svg/CSA.svg
                let svg = svg.replace(
                    r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"##,
                    "",
                );

                // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/xlink:href
                let svg = svg.replace("xlink:href", "href");
                let svg = svg.replace(r#"xmlns:xlink="http://www.w3.org/1999/xlink""#, "");

                let svg = if true || feature_name == "lucide" {
                    optimize(&svg)
                } else {
                    svg
                };

                bar.inc(1);

                // warning: The tag 'clipPath' is not matching its normalized form 'clippath'. If you
                // want to keep this form, change this to a dynamic tag `@{"clipPath"}`.
                //let svg = svg.replace("<clipPath ", "<clippath ");
                //let svg = svg.replace("</clipPath>", "</clippath>");

                let (first_tag, remainder) = svg.split_once('>').unwrap();
                let mut first_tag = first_tag.to_owned() + ">";
                let (remainder, _last_tag) = remainder.rsplit_once('<').unwrap();
                let remainder = remainder.to_owned();

                assert!(first_tag.len() > 5, "{}", first_tag);

                if first_tag.contains(" width=") {
                    first_tag = width_regex.replace(&first_tag, "").into_owned();
                }
                if first_tag.contains(" height=") {
                    first_tag = height_regex.replace(&first_tag, "").into_owned();
                }
                if first_tag.contains(" role=") {
                    first_tag = role_regex.replace(&first_tag, "").into_owned();
                }

                let remainder = title_regex.replace_all(&remainder, "").into_owned();
                let remainder = desc_regex.replace_all(&remainder, "").into_owned();

                // Yew's [`html!`] macro doesn't support comments.
                let remainder = comment_regex.replace_all(&remainder, "").into_owned();

                // Yew's [`html!`] macro requires quoted, bracketed strings.
                let remainder = text_regex
                    .replace_all(&remainder, r##">{"$1"}<"##)
                    .into_owned();

                let mut replacement = format!(
                    r#"xmlns="http://www.w3.org/2000/svg""#,
                );

                if !svg.contains("stroke=") && !svg.contains("fill=") {
                    replacement += r#" fill="currentColor""#;

                    /*
                    if svg.contains("fill-rule") || prefix == "Octicons" {
                        replacement += r#" fill="currentColor""#;
                    } else {
                        replacement += r#" stroke="currentColor""#;
                    }
                    */
                }

                first_tag = first_tag.replace(r#"xmlns="http://www.w3.org/2000/svg""#, &replacement);

                let feature_name = feature_name.clone();

                let icon_name = icon_name.to_case(Case::UpperSnake);

                let constant_name = name.to_case(Case::UpperSnake);

                let ret: Box<dyn Fn() -> (TokenStream, TokenStream) + Send> = Box::new(move || {
                    let mut parse: &str = first_tag.as_str();
                    parse = parse.strip_prefix("<svg").unwrap();
                    parse = parse.strip_suffix(">").unwrap();
    
                    let mut view_box_attr = quote!{None};
                    let mut fill_attr = quote!{None};
                    let mut stroke_attr = quote!{None};
                    let mut stroke_width_attr = quote!{None};
                    let mut stroke_linecap_attr = quote!{None};
                    let mut stroke_linejoin_attr = quote!{None};
                    let src_attr = remainder.trim();
    
                    loop {
                        if parse.is_empty() {
                            break;
                        }
                        let (k, next) = parse.split_once('=').expect(parse);
                        let key = k.trim();
                        parse = next.trim().strip_prefix("\"").unwrap();
    
                        let (value, next) = parse.split_once('"').expect(parse);
                        parse = next.trim();
    
                        let attr = match key {
                            "xmlns" => {
                                assert_eq!(value, "http://www.w3.org/2000/svg");
                                None
                            }
                            "viewBox" => Some(&mut view_box_attr),
                            "fill" => Some(&mut fill_attr),
                            "stroke" => Some(&mut stroke_attr),
                            "stroke-width" => Some(&mut stroke_width_attr),
                            "stroke-linecap" => Some(&mut stroke_linecap_attr),
                            "stroke-linejoin" => Some(&mut stroke_linejoin_attr),
                            "id" | "version" | "aria-hidden" => {
                                None
                            }
                            _ => panic!("{key}")
                        };
    
                        if let Some(attr) = attr {
                            *attr = quote!{Some(#value)};
                        }
                    }
    
                    let constant = to_ident(&constant_name);

                    let icon_data_item = quote! {
                        pub const #constant: Self = {
                            Self {
                                collection: &COLLECTION,
                                name: #icon_name,
                                view_box: #view_box_attr,
                                fill: #fill_attr,
                                stroke: #stroke_attr,
                                stroke_width: #stroke_width_attr,
                                stroke_linecap: #stroke_linecap_attr,
                                stroke_linejoin: #stroke_linejoin_attr,
                                src: #src_attr,
                            }
                        };
                    };
    
                    let enumerate_item = quote! {
                        #[cfg(feature = #feature_name)]
                        Self::#constant,
                    };
    
                    (icon_data_item, enumerate_item)
                });
                ret
            })
            .collect::<Vec<Box<dyn Fn() -> (TokenStream, TokenStream) + Send>>>();

        bar.finish();

        for func in items {
            let (icon_data_item, enumerate_item) = func();
            icon_data.push(icon_data_item);
            enumerate.push(enumerate_item);
        }

        imports.push(quote! {
            #[cfg(feature = #feature_name)]
            mod #feature_ident;
        });

        let collection_name = feature_name.to_case(Case::UpperSnake);

        let tokens = quote! {
            static COLLECTION: crate::IconCollection = crate::IconCollection{
                name: #collection_name,
                license: #license,
            };

            impl crate::IconData {
                #(#icon_data)*
            }
        };

        let output = reformat(tokens.to_string(), true).unwrap();
        write(format!("src/generated/{}.rs", feature_name), output).unwrap();

        features.push(feature_name);
    };

    let font_awesome_license = r##"Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc."##;
    let heroicons_license =
        r##"From https://github.com/tailwindlabs/heroicons - Licensed under MIT"##;

    generate(
        "Bootstrap",
        "bootstrap/icons",
        r##"From https://github.com/twbs/icons - Licensed under MIT"##,
    );
    generate(
        "Feather",
        "feather/icons",
        r##"From https://github.com/feathericons/feather - Licensed under MIT"##,
    );
    generate(
        "LipisFlagIcons1x1",
        "lipis-flag-icons/flags/1x1",
        r##"From https://github.com/lipis/flag-icons - Licensed under MIT"##,
    );
    generate(
        "LipisFlagIcons4x3",
        "lipis-flag-icons/flags/4x3",
        r##"From https://github.com/lipis/flag-icons - Licensed under MIT"##,
    );
    /*
    generate(
        "CountryFlagIcons",
        "country-flag-icons/3x2/",
        r##"From https://github.com/catamphetamine/country-flag-icons - Licensed under MIT"##,
    );
    generate(
        "YammadevFlagIcons",
        "yammadev-flag-icons/svg",
        r##"From https://github.com/yammadev/flag-icons - Licensed under MIT"##,
    );
     */
    generate(
        "FontAwesomeRegular",
        "Font-Awesome/svgs/regular",
        font_awesome_license,
    );
    generate(
        "FontAwesomeSolid",
        "Font-Awesome/svgs/solid",
        font_awesome_license,
    );
    generate(
        "HeroiconsOutline",
        "heroicons/optimized/24/outline",
        heroicons_license,
    );
    generate(
        "HeroiconsSolid",
        "heroicons/optimized/24/solid",
        heroicons_license,
    );
    generate(
        "HeroiconsMiniSolid",
        "heroicons/optimized/20/solid",
        heroicons_license,
    );
    generate(
        "Lucide",
        "lucide/icons",
        r##"From https://github.com/lucide-icons/lucide - Licensed under ISC"##,
    );
    generate(
        "Octicons",
        "octicons/icons",
        r##"From https://github.com/primer/octicons - (c) GitHub, Inc."##,
    );
    generate(
        "SimpleIcons",
        "simple-icons/icons",
        r##"From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines"##,
    );
    generate("Extra", "extra", r##"Check brand guidelines"##);

    let tokens = quote! {
        #(#imports)*

        #[cfg(feature = "_enumerate_icon_data")]
        impl crate::IconData {
            #[doc(hidden)]
            pub const ENUMERATE : &'static [Self] = &[
                #(#enumerate)*
            ];
        }
    };

    let output = reformat(tokens.to_string(), false).unwrap();

    write("src/generated.rs", output).unwrap();

    features.sort_unstable_by_key(|feature| feature.clone());

    for feature in features {
        println!(r##"{} = []"##, feature)
    }
}

fn to_ident(string: &str) -> Ident {
    Ident::new(string, Span::call_site())
}

// https://github.com/rust-analyzer/rust-analyzer/blob/ada9e16537c22b490d13cdd54b9e1e4885856a4c/xtask/src/codegen.rs#L66-L78
fn reformat(text: impl std::fmt::Display, included: bool) -> Result<String, String> {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    write!(rustfmt.stdin.take().unwrap(), "{}", text).map_err(|e| e.to_string())?;
    let output = rustfmt.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let preamble = "Generated file, do not edit by hand, see `src/generator.rs`";
    let prefix = if included { "//" } else { "//!" };
    Ok(format!("{} {}\n\n{}", prefix, preamble, stdout))
}

fn optimize(svg: &str) -> String {
    let mut input = tempfile::NamedTempFile::new().unwrap();
    let output = tempfile::NamedTempFile::new().unwrap();

    input.as_file_mut().write_all(svg.as_bytes()).unwrap();
    //input.as_file_mut().flush().unwrap();

    let mut command = Command::new("svgo");
    command
        .arg("--config")
        .arg("./svgo.config.js")
        .arg("--input")
        .arg(input.path())
        .arg("-p")
        .arg("4")
        .arg("--multipass")
        .arg("--output")
        .arg(output.path());

    let result = command.output().unwrap();

    drop(input);

    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));

    let mut ret = String::new();
    output.as_file().read_to_string(&mut ret).unwrap();
    //println!("{ret}");
    ret
}