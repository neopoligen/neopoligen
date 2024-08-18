use crate::neo_error::{NeoError, NeoErrorKind};
use crate::payload_span::PayloadSpan;
use crate::span::*;
use minijinja::Value;
use regex::Regex;
use rimage::config::{Codec, EncoderConfig};
use rimage::image::imageops::FilterType;
use rimage::image::DynamicImage;
use rimage::Decoder;
use rimage::Encoder;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use walkdir::WalkDir;

pub fn empty_dir(dir: &PathBuf) -> std::io::Result<()> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

pub fn flatten_payload_spans(spans: &Vec<PayloadSpan>) -> String {
    spans
        .iter()
        .map(|span| flatten_payload_parsed_text(span))
        .collect::<Vec<String>>()
        .join("")
}

// TODO: Deal with nestings
pub fn flatten_payload_parsed_text(span: &PayloadSpan) -> String {
    span.parsed_text.clone()
}

pub fn flatten_spans(spans: &Vec<Span>) -> String {
    spans
        .iter()
        .map(|span| flatten_parsed_text(span))
        .collect::<Vec<String>>()
        .join("")
}

pub fn flatten_parsed_text(span: &Span) -> String {
    match &span.kind {
        SpanKind::NamedSpan { children, .. } => children
            .iter()
            .map(|span| flatten_parsed_text(span))
            .collect::<Vec<String>>()
            .join(""),
        _ => span.parsed_text.clone(),
    }
}

pub fn format_html_for_theme_test_display(code: &str) -> String {
    let mut re = Regex::new(r"\n").unwrap();
    let output = re.replace_all(code, " ");
    re = Regex::new(r" \s+").unwrap();
    let output = re.replace_all(&output, " ");
    re = Regex::new(r"\s+<").unwrap();
    let output = re.replace_all(&output, "<");
    re = Regex::new(r">\s+").unwrap();
    let output = re.replace_all(&output, ">");
    let parts: Vec<&str> = output.split("<").collect();
    let mut assembler: Vec<String> = vec![];
    let mut level = 0i8;
    assembler.push(parts[0].to_string());
    parts.iter().skip(1).for_each(|part| {
        if part.starts_with("/") {
            level -= 2;
        }
        for _ in 0..level {
            assembler.push(" ".to_string());
        }
        assembler.push(format!("<{}\n", part));
        if !part.starts_with("/") {
            level += 2;
        }
    });
    assembler.join("").to_string()
}

pub fn get_dirs_in_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, NeoError> {
    if !dir.exists() {
        Err(NeoError {
            kind: NeoErrorKind::GenericErrorWithSourcePath {
                source_path: dir.clone(),
                msg: "Could not read directory to get directories".to_string(),
            },
        })
    } else {
        if let Ok(the_dir) = fs::read_dir(dir) {
            Ok(the_dir
                .filter_map(|e| {
                    let path = e.unwrap().path().to_path_buf();
                    if path.is_dir() {
                        Some(path)
                    } else {
                        None
                    }
                })
                .collect())
        } else {
            Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithSourcePath {
                    source_path: dir.clone(),
                    msg: "Could not read directory".to_string(),
                },
            })
        }

        //         fs::read_dir(dir)?
        //            .map(|entry| {
        //                let entry = entry?;
        //               Ok(entry)
        //         })
        //         .filter_map(|entry: Result<DirEntry, io::Error>| {
        //             let path = entry.unwrap().path();
        //             if path.is_dir() {
        //                 match path.file_name() {
        //                     Some(file_name) => {
        //                         if file_name.to_string_lossy().starts_with(".") {
        //                             None
        //                         } else {
        //                             Some(Ok(path))
        //                         }
        //                     }
        //                     None => None,
        //                 }
        //             } else {
        //                 None
        //             }
        //         }),
        // )
    }

    // Result::from_iter(
    //     fs::read_dir(dir)?
    //         .map(|entry| {
    //             let entry = entry?;
    //             Ok(entry)
    //         })
    //         .filter_map(|entry: Result<DirEntry, io::Error>| {
    //             let path = entry.unwrap().path();
    //             if path.is_dir() {
    //                 match path.file_name() {
    //                     Some(file_name) => {
    //                         if file_name.to_string_lossy().starts_with(".") {
    //                             None
    //                         } else {
    //                             Some(Ok(path))
    //                         }
    //                     }
    //                     None => None,
    //                 }
    //             } else {
    //                 None
    //             }
    //         }),
    // )
}

pub fn get_files_with_extension_in_a_single_directory(
    dir: &PathBuf,
    extension: &str,
) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter(|p| {
            if p.as_ref().unwrap().path().is_file() {
                true
            } else {
                false
            }
        })
        .filter(|p| match p.as_ref().unwrap().path().extension() {
            Some(ext) => ext == extension,
            None => false,
        })
        .filter_map(|p| match p.as_ref().unwrap().path().strip_prefix(".") {
            Ok(_) => None,
            Err(_) => Some(p.as_ref().unwrap().path()),
        })
        .collect()
}

pub fn get_image_paths(source_dir: &PathBuf) -> Vec<PathBuf> {
    let walker = WalkDir::new(source_dir).into_iter();
    walker
        .filter_map(|path_result| match path_result {
            Ok(path) => match path.path().extension() {
                Some(ext) => {
                    if ext.to_ascii_lowercase().eq("jpg") {
                        Some(path.path().to_path_buf())
                    } else if ext.to_ascii_lowercase().eq("jepg") {
                        Some(path.path().to_path_buf())
                    } else if ext.to_ascii_lowercase().eq("png") {
                        Some(path.path().to_path_buf())
                    } else {
                        None
                    }
                }
                None => None,
            },
            Err(_) => None,
        })
        .collect()
}

pub fn get_neo_files_in_dir_recursively(dir: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path().to_path_buf();
                if path.is_file() {
                    if let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) {
                        if ext.to_ascii_lowercase() == "neo"
                            && !filename.to_str().unwrap().starts_with(".")
                        {
                            Some(path.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

pub fn highlight_code(args: &[Value]) -> String {
    let code = args[0].to_string();
    let lang = args[1].to_string();
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set
        .find_syntax_by_token(&lang)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in LinesWithEndings::from(&trim_empty_lines(&code)) {
        let _ = html_generator.parse_html_for_line_which_includes_newline(line);
    }
    let initial_html = html_generator.finalize();
    let output_html: Vec<_> = initial_html
        .lines()
        .map(|line| format!(r#"<span class="line-marker"></span>{}"#, line))
        .collect();
    output_html.join("\n")
}

pub fn highlight_span(args: &[Value]) -> String {
    let code = args[0].to_string();
    dbg!(&code);
    let lang = args[1].to_string();
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set
        .find_syntax_by_token(&lang)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in LinesWithEndings::from(&code) {
        let _ = html_generator.parse_html_for_line_which_includes_newline(line);
    }
    let initial_html = html_generator.finalize();
    let output_html: Vec<_> = initial_html
        .lines()
        .map(|line| format!(r#"{}"#, line))
        .collect();
    output_html.join("\n")
}

pub fn resize_and_optimize_jpg(
    source: &PathBuf,
    width: u32,
    dest: &PathBuf,
) -> Result<(), NeoError> {
    match Decoder::from_path(source) {
        Ok(decoder) => match decoder.decode() {
            Ok(image) => {
                let height = image.height() * width / image.width();
                let resized_image = image.resize_to_fill(width, height, FilterType::Lanczos3);
                let config = EncoderConfig::new(Codec::MozJpeg)
                    .with_quality(90.0)
                    .unwrap();
                match File::create(&dest) {
                    Ok(file) => {
                        let encoder =
                            Encoder::new(file, DynamicImage::ImageRgba8(resized_image.into()))
                                .with_config(config);
                        match encoder.encode() {
                            Ok(_) => Ok(()),
                            Err(e) => Err(NeoError {
                                kind: NeoErrorKind::GenericErrorWithSourcePath {
                                    source_path: source.clone(),
                                    msg: format!("image processing error: {}", e),
                                },
                            }),
                        }
                    }
                    Err(e) => Err(NeoError {
                        kind: NeoErrorKind::GenericErrorWithSourcePath {
                            source_path: source.clone(),
                            msg: format!("image processing error: {}", e),
                        },
                    }),
                }
            }
            Err(e) => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithSourcePath {
                    source_path: source.clone(),
                    msg: format!("image processing error: {}", e),
                },
            }),
        },
        Err(e) => Err(NeoError {
            kind: NeoErrorKind::GenericErrorWithSourcePath {
                source_path: source.clone(),
                msg: format!("image processing error: {}", e),
            },
        }),
    }
}

pub fn resize_and_optimize_png(
    source: &PathBuf,
    width: u32,
    dest: &PathBuf,
) -> Result<(), NeoError> {
    match Decoder::from_path(source) {
        Ok(decoder) => match decoder.decode() {
            Ok(image) => {
                let height = image.height() * width / image.width();
                let resized_image = image.resize_to_fill(width, height, FilterType::Lanczos3);
                let config = EncoderConfig::new(Codec::OxiPng);
                match File::create(&dest) {
                    Ok(file) => {
                        let encoder =
                            Encoder::new(file, DynamicImage::ImageRgba8(resized_image.into()))
                                .with_config(config);
                        match encoder.encode() {
                            Ok(_) => Ok(()),
                            Err(e) => Err(NeoError {
                                kind: NeoErrorKind::GenericErrorWithSourcePath {
                                    source_path: source.clone(),
                                    msg: format!("image processing error: {}", e),
                                },
                            }),
                        }
                    }
                    Err(e) => Err(NeoError {
                        kind: NeoErrorKind::GenericErrorWithSourcePath {
                            source_path: source.clone(),
                            msg: format!("image processing error: {}", e),
                        },
                    }),
                }
            }
            Err(e) => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithSourcePath {
                    source_path: source.clone(),
                    msg: format!("image processing error: {}", e),
                },
            }),
        },
        Err(e) => Err(NeoError {
            kind: NeoErrorKind::GenericErrorWithSourcePath {
                source_path: source.clone(),
                msg: format!("image processing error: {}", e),
            },
        }),
    }
}

pub fn scrub_rel_file_path(source: &str) -> Result<PathBuf, NeoError> {
    let pb = PathBuf::from(source);
    let pb = if pb.starts_with("/") {
        match pb.strip_prefix("/") {
            Ok(p) => p.to_path_buf(),
            Err(e) => {
                return Err(NeoError {
                    kind: NeoErrorKind::GenericErrorWithoutSourcePath { msg: e.to_string() },
                })
            }
        }
    } else {
        pb.to_path_buf()
    };
    Ok(pb.join("index.html"))
}

pub fn trim_empty_lines(source: &str) -> String {
    let re = Regex::new(r"\S").unwrap();
    let trimmed_front = source.split("\n").fold("".to_string(), |acc, l| {
        if !acc.is_empty() {
            acc + l + "\n"
        } else {
            if re.is_match(l) {
                l.to_string() + "\n"
            } else {
                acc
            }
        }
    });
    trimmed_front.trim_end().to_string()
}

// TODO: Add test to remove "quot" if there's a ``&quot;``
// in the string
pub fn url_from_string(source: &str) -> String {
    let re1 = Regex::new(r"\W").unwrap();
    let re2 = Regex::new(r"-+").unwrap();
    let re3 = Regex::new(r"^-").unwrap();
    let re4 = Regex::new(r"-$").unwrap();
    let mut updated = source.to_lowercase();
    updated = re1.replace_all(&updated, "-").to_string();
    updated = re2.replace_all(&updated, "-").to_string();
    updated = re3.replace_all(&updated, "").to_string();
    updated = re4.replace_all(&updated, "").to_string();
    updated.to_string()
}

// TODO: Switch to NeoError
pub fn write_file_with_mkdir(path: &PathBuf, content: &str) -> Result<(), String> {
    match path.parent() {
        Some(parent_dir) => match fs::create_dir_all(parent_dir) {
            Ok(_) => match fs::write(path, content) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        None => Err("Could not make directory".to_string()),
    }
}
