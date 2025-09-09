use crate::neo_error::{NeoError, NeoErrorKind};
use crate::span_v42::SpanV42;
use anyhow::Result as AnyResult;
use include_dir::Dir;
use minijinja::Error;
use minijinja::ErrorKind;
use minijinja::Value;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::UNIX_EPOCH;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use walkdir::WalkDir;

pub fn copy_file(
  source_path: &PathBuf,
  dest_path: &PathBuf,
) -> Result<(), std::io::Error> {
  if source_path.is_file() {
    let dest_dir = dest_path.parent().unwrap();
    fs::create_dir_all(dest_dir)?;
    fs::copy(source_path, dest_path)?;
  }
  Ok(())
}

pub fn copy_folder(
  source_folder: &PathBuf,
  dest_folder: &PathBuf,
) -> Result<(), std::io::Error> {
  for entry in WalkDir::new(source_folder) {
    let source_path = entry?.into_path();
    let dest_path = dest_folder
      .join(source_path.strip_prefix(source_folder).unwrap());
    if source_path.is_dir() {
      fs::create_dir_all(dest_path)?;
    } else {
      let data = std::fs::read(source_path)?;
      std::fs::write(dest_path, &data)?;
    }
  }
  Ok(())
}

pub fn dump_parser_if_debug(
  debug: bool,
  dbg_header: &str,
  source: &str,
  count: usize,
) {
  if debug {
    println!("");
    for _ in 0..count * 2 {
      print!("█");
    }
    println!(" {}: {}", dbg_header, count);
    let l = source
      .lines()
      .take(5)
      .map(|line| line)
      .collect::<Vec<&str>>()
      .join(r#"\n"#);
    for _ in 0..count * 2 {
      print!("█");
    }
    println!(" {}", l);
  }
}

pub fn empty_folder(dir: &PathBuf) -> std::io::Result<()> {
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

pub fn file_epoch_mod_time(
  path: &PathBuf
) -> anyhow::Result<u64> {
  let base_mod_time = fs::metadata(path)?
    .modified()?
    .duration_since(UNIX_EPOCH)?
    .as_secs();
  Ok(base_mod_time)
}

pub fn flatten_spans_v42(spans: &Vec<SpanV42>) -> String {
  spans
    .iter()
    .map(|span| match span {
      SpanV42::PlainText(s) => s.text.to_string(),
      SpanV42::NamedSpan(s) => match &s.spans.clone() {
        Some(spans) => flatten_spans_v42(spans),
        None => "".to_string(),
      },
      SpanV42::CodeShorthand(s) => s.text.to_string(),
      SpanV42::EmphasisShorthand(s) => s.text.to_string(),
      SpanV42::FootnoteShorthand(s) => s.text.to_string(),
      SpanV42::StrongShorthand(s) => s.text.to_string(),
      SpanV42::EscapedCharacter(s) => s.text.to_string(),
      _ => "TODO_IN_HELPERS_MOD".to_string(),
    })
    .collect::<Vec<String>>()
    .join("")
}

// TODO: Fix this so it won't crash if you send in something
// other than spans
pub fn flatten_spans_from_template_v42(
  args: &[Value]
) -> Result<Value, Error> {
  if args.len() == 1 {
    let x: &String = &args[0]
      .try_iter()?
      .map(|v| {
        v.get_item(&Value::from("text")).unwrap().to_string()
      })
      .collect::<Vec<_>>()
      .join("");
    Ok(Value::from(x))
  } else {
    Err(Error::new(
      ErrorKind::InvalidOperation,
      "Missing argument for flatten_spans in template"
        .to_string(),
    ))
  }
}

pub fn format_html_for_theme_test_display(
  code: &str
) -> String {
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

pub fn get_folders_in_folder(
  dir: &PathBuf
) -> Result<Vec<PathBuf>, NeoError> {
  if !dir.exists() {
    Err(NeoError {
      kind: NeoErrorKind::GenericErrorWithSourcePath {
        source_path: dir.clone(),
        msg: "Could not read directory to get directories"
          .to_string(),
      },
    })
  } else {
    if let Ok(the_folder) = fs::read_dir(dir) {
      Ok(
        the_folder
          .filter_map(|e| {
            let path = e.unwrap().path().to_path_buf();
            if path.is_dir() {
              Some(path)
            } else {
              None
            }
          })
          .collect(),
      )
    } else {
      Err(NeoError {
        kind: NeoErrorKind::GenericErrorWithSourcePath {
          source_path: dir.clone(),
          msg: "Could not read directory".to_string(),
        },
      })
    }
    //         fs::read_folder(dir)?
    //            .map(|entry| {
    //                let entry = entry?;
    //               Ok(entry)
    //         })
    //         .filter_map(|entry: Result<DirEntry, io::Error>| {
    //             let path = entry.unwrap().path();
    //             if path.is_folder() {
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
  //     fs::read_folder(dir)?
  //         .map(|entry| {
  //             let entry = entry?;
  //             Ok(entry)
  //         })
  //         .filter_map(|entry: Result<DirEntry, io::Error>| {
  //             let path = entry.unwrap().path();
  //             if path.is_folder() {
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

pub fn get_files_with_extension_in_a_single_folder(
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
    .filter_map(|p| {
      match p.as_ref().unwrap().path().strip_prefix(".") {
        Ok(_) => None,
        Err(_) => Some(p.as_ref().unwrap().path()),
      }
    })
    .collect()
}

pub fn get_include_str_file(
  dir: Dir,
  name: &str,
) -> String {
  match dir.get_file(name) {
    Some(data) => data.contents_utf8().unwrap().to_string(),
    None => {
      dbg!(format!("Could not open file: {}", name));
      "".to_string()
    }
  }
}

pub fn get_minijinja_error_text(
  err: &minijinja::Error
) -> Vec<String> {
  let mut parts: Vec<String> = vec![];
  parts.push(format!("{:#}", err));
  let mut err = &err as &dyn std::error::Error;
  while let Some(next_err) = err.source() {
    parts.push(format!("{:#}", next_err));
    err = next_err;
  }
  parts
}

pub fn get_neo_files_in_folder_recursively(
  dir: &PathBuf
) -> Vec<PathBuf> {
  WalkDir::new(dir)
    .into_iter()
    .filter_map(|entry| {
      if let Ok(entry) = entry {
        let path = entry.path().to_path_buf();
        if path.is_file() {
          if let (Some(filename), Some(ext)) =
            (path.file_name(), path.extension())
          {
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

pub fn get_string_as_sha256_hash(
  input: &String
) -> AnyResult<String> {
  let mut hasher = Sha256::new();
  hasher.update(input.as_bytes());
  let result: String = format!("{:X}", hasher.finalize());
  Ok(result)
}

pub fn highlight_code_section(args: &[Value]) -> String {
  highlight_code(args)
}

// TODO: Move this to highlight_code_section
pub fn highlight_code(args: &[Value]) -> String {
  let code = args[0].to_string();
  let lang = args[1].to_string();
  let start = if args.len() == 4 {
    args[2].to_string().parse::<usize>().unwrap()
  } else {
    0
  };
  let end = if args.len() == 4 {
    args[3].to_string().parse::<usize>().unwrap() - start + 1
  } else {
    0
  };
  let syntax_set = SyntaxSet::load_defaults_newlines();
  let syntax = syntax_set
    .find_syntax_by_token(&lang)
    .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
  let mut html_generator =
    ClassedHTMLGenerator::new_with_class_style(
      syntax,
      &syntax_set,
      ClassStyle::Spaced,
    );
  for line in LinesWithEndings::from(&trim_empty_lines(&code)) {
    let _ = html_generator
      .parse_html_for_line_which_includes_newline(line);
  }
  let initial_html = html_generator.finalize();
  let output_html: Vec<_> = initial_html
    .lines()
    .map(|line| {
      format!(
        r#"<span class="aws-code-block-marker"></span>{}"#,
        line
      )
    })
    .collect();
  if start > 0 {
    output_html
      .iter()
      .skip(start - 1)
      .take(end)
      .map(|x| x.to_string())
      .collect::<Vec<String>>()
      .join("\n")
  } else {
    output_html.join("\n")
  }
}

// TODO: Remove this in favor of handling the
// nums v no-nums in the templates and CSS.
pub fn highlight_code_no_nums(args: &[Value]) -> String {
  let code = args[0].to_string();
  let lang = args[1].to_string();
  let start = if args.len() == 4 {
    args[2].to_string().parse::<usize>().unwrap()
  } else {
    0
  };
  let end = if args.len() == 4 {
    args[3].to_string().parse::<usize>().unwrap() - start + 1
  } else {
    0
  };
  let syntax_set = SyntaxSet::load_defaults_newlines();
  let syntax = syntax_set
    .find_syntax_by_token(&lang)
    .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
  let mut html_generator =
    ClassedHTMLGenerator::new_with_class_style(
      syntax,
      &syntax_set,
      ClassStyle::Spaced,
    );
  for line in LinesWithEndings::from(&trim_empty_lines(&code)) {
    let _ = html_generator
      .parse_html_for_line_which_includes_newline(line);
  }
  let initial_html = html_generator.finalize();
  // NOTE: This may not be the most efficient way to make the lines
  // since there's no addition like in the regular highlight_code()
  // function, but it works fine.
  let output_html: Vec<_> = initial_html
    .lines()
    .map(|line| format!(r#"{}"#, line))
    .collect();
  if start > 0 {
    output_html
      .iter()
      .skip(start - 1)
      .take(end)
      .map(|x| x.to_string())
      .collect::<Vec<String>>()
      .join("\n")
  } else {
    output_html.join("\n")
  }
}

pub fn highlight_file(
  content_root: PathBuf,
  path_string: String,
  lang: String,
) -> String {
  let path = content_root.join(path_string);
  if path_exists(&path) {
    match fs::read_to_string(&path) {
      Ok(code) => {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let syntax = syntax_set
          .find_syntax_by_token(&lang)
          .unwrap_or_else(|| {
            syntax_set.find_syntax_plain_text()
          });
        let mut html_generator =
          ClassedHTMLGenerator::new_with_class_style(
            syntax,
            &syntax_set,
            ClassStyle::Spaced,
          );
        for line in
          LinesWithEndings::from(&trim_empty_lines(&code))
        {
          let _ = html_generator
            .parse_html_for_line_which_includes_newline(line);
        }
        let initial_html = html_generator.finalize();
        let output_html: Vec<_> = initial_html
              .lines()
              .map(|line| {
                format!(
                  r#"<span class="aws-code-block-marker"></span>{}"#,
                  line
                )
              })
              .collect();
        output_html.join("\n")
      }
      Err(_e) => {
        format!("could not read file: {}", &path.display())
      }
    }
  } else {
    format!(
      "No file exists at: {} - The site dir is currently hard coded",
      path.display(),
    )
  }
}

pub fn highlight_span_v42(
  code: &str,
  lang: &str,
) -> String {
  let syntax_set = SyntaxSet::load_defaults_newlines();
  let syntax = syntax_set
    .find_syntax_by_token(lang)
    .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
  let mut html_generator =
    ClassedHTMLGenerator::new_with_class_style(
      syntax,
      &syntax_set,
      ClassStyle::Spaced,
    );
  for line in LinesWithEndings::from(code) {
    let _ = html_generator
      .parse_html_for_line_which_includes_newline(line);
  }
  let initial_html = html_generator.finalize();
  let output_html: Vec<_> = initial_html
    .lines()
    .map(|line| format!(r#"{}"#, line))
    .collect();
  output_html.join("\n")
}

// TODO: Deprecate in favor of V42
pub fn highlight_span(args: &[Value]) -> String {
  let code = args[0].to_string();
  let lang = args[1].to_string();
  let syntax_set = SyntaxSet::load_defaults_newlines();
  let syntax = syntax_set
    .find_syntax_by_token(&lang)
    .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
  let mut html_generator =
    ClassedHTMLGenerator::new_with_class_style(
      syntax,
      &syntax_set,
      ClassStyle::Spaced,
    );
  for line in LinesWithEndings::from(&code) {
    let _ = html_generator
      .parse_html_for_line_which_includes_newline(line);
  }
  let initial_html = html_generator.finalize();
  let output_html: Vec<_> = initial_html
    .lines()
    .map(|line| format!(r#"{}"#, line))
    .collect();
  output_html.join("\n")
}

pub fn mkdir_p(dir: PathBuf) -> std::io::Result<()> {
  if dir.exists() {
    Ok(())
  } else {
    std::fs::create_dir_all(dir)
  }
}

pub fn path_exists(path: &PathBuf) -> bool {
  match path.try_exists() {
    Ok(exists) => {
      if exists == true {
        true
      } else {
        false
      }
    }
    Err(_) => false,
  }
}

pub fn replace_path(
  path: &PathBuf,
  find: &PathBuf,
  replace: &PathBuf,
) -> Result<PathBuf, String> {
  match path.strip_prefix(find) {
    Ok(path_part) => Ok(replace.clone().join(path_part)),
    Err(e) => Err(format!("Could not reaplce path: {}", e)),
  }
}

pub fn resize_and_optimize_image(
  source: &PathBuf,
  width: u32,
  dest: &PathBuf,
) {
  // dbg!(&source);
  // dbg!(&dest);
  match &dest.parent() {
    Some(parent_folder) => {
      match fs::create_dir_all(parent_folder) {
        Ok(_) => {
          let source_image_path =
            &source.to_string_lossy().to_string();
          // dbg!(&source_image_path);
          let width_as_string = &width.to_string();
          let args: Vec<&str> = vec![
            source_image_path,
            "-resize",
            width_as_string,
            &dest.to_str().unwrap(), // magick input.jpg -resize 50% output.jpg
          ];
          let _ =
            Command::new("magick").args(args).output().unwrap();
          // dbg!(cmd_output);
        }
        _ => (),
      }
    }
    _ => (),
  }
}

// // deprecate in favor of resize and optimize image
// pub fn resize_and_optimize_jpg(
//     source: &PathBuf,
//     width: u32,
//     dest: &PathBuf,
// ) -> Result<(), NeoError> {
//     match Decoder::from_path(source) {
//         Ok(decoder) => match decoder.decode() {
//             Ok(image) => {
//                 let height = image.height() * width / image.width();
//                 let resized_image = image.resize_to_fill(width, height, FilterType::Lanczos3);
//                 let config = EncoderConfig::new(Codec::MozJpeg)
//                     .with_quality(90.0)
//                     .unwrap();
//                 match &dest.parent() {
//                     Some(parent_folder) => match fs::create_dir_all(parent_folder) {
//                         _ => (),
//                     },
//                     _ => (),
//                 }
//                 match File::create(&dest) {
//                     Ok(file) => {
//                         let encoder =
//                             Encoder::new(file, DynamicImage::ImageRgba8(resized_image.into()))
//                                 .with_config(config);
//                         match encoder.encode() {
//                             Ok(_) => Ok(()),
//                             Err(e) => Err(NeoError {
//                                 kind: NeoErrorKind::GenericErrorWithSourcePath {
//                                     source_path: source.clone(),
//                                     msg: format!("image processing error: {}", e),
//                                 },
//                             }),
//                         }
//                     }
//                     Err(e) => Err(NeoError {
//                         kind: NeoErrorKind::GenericErrorWithSourcePath {
//                             source_path: source.clone(),
//                             msg: format!("image processing error: {}", e),
//                         },
//                     }),
//                 }
//             }
//             Err(e) => Err(NeoError {
//                 kind: NeoErrorKind::GenericErrorWithSourcePath {
//                     source_path: source.clone(),
//                     msg: format!("image processing error: {}", e),
//                 },
//             }),
//         },
//         Err(e) => Err(NeoError {
//             kind: NeoErrorKind::GenericErrorWithSourcePath {
//                 source_path: source.clone(),
//                 msg: format!("image processing error: {}", e),
//             },
//         }),
//     }
// }

// // deprecate in favor of resize and optimize image
// pub fn resize_and_optimize_png(
//     source: &PathBuf,
//     width: u32,
//     dest: &PathBuf,
// ) -> Result<(), NeoError> {
//     match Decoder::from_path(source) {
//         Ok(decoder) => match decoder.decode() {
//             Ok(image) => {
//                 let height = image.height() * width / image.width();
//                 let resized_image = image.resize_to_fill(width, height, FilterType::Lanczos3);
//                 let config = EncoderConfig::new(Codec::OxiPng);
//                 match File::create(&dest) {
//                     Ok(file) => {
//                         let encoder =
//                             Encoder::new(file, DynamicImage::ImageRgba8(resized_image.into()))
//                                 .with_config(config);
//                         match encoder.encode() {
//                             Ok(_) => Ok(()),
//                             Err(e) => Err(NeoError {
//                                 kind: NeoErrorKind::GenericErrorWithSourcePath {
//                                     source_path: source.clone(),
//                                     msg: format!("image processing error: {}", e),
//                                 },
//                             }),
//                         }
//                     }
//                     Err(e) => Err(NeoError {
//                         kind: NeoErrorKind::GenericErrorWithSourcePath {
//                             source_path: source.clone(),
//                             msg: format!("image processing error: {}", e),
//                         },
//                     }),
//                 }
//             }
//             Err(e) => Err(NeoError {
//                 kind: NeoErrorKind::GenericErrorWithSourcePath {
//                     source_path: source.clone(),
//                     msg: format!("image processing error: {}", e),
//                 },
//             }),
//         },
//         Err(e) => Err(NeoError {
//             kind: NeoErrorKind::GenericErrorWithSourcePath {
//                 source_path: source.clone(),
//                 msg: format!("image processing error: {}", e),
//             },
//         }),
//     }
// }

pub fn scrub_rel_file_path(
  source: &str
) -> Result<PathBuf, NeoError> {
  let pb = PathBuf::from(source);
  let pb = if pb.starts_with("/") {
    match pb.strip_prefix("/") {
      Ok(p) => p.to_path_buf(),
      Err(e) => {
        return Err(NeoError {
          kind: NeoErrorKind::GenericErrorWithoutSourcePath {
            msg: e.to_string(),
          },
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
  let trimmed_front =
    source.split("\n").fold("".to_string(), |acc, l| {
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
pub fn write_file_with_mkdir(
  path: &PathBuf,
  content: &str,
) -> Result<(), String> {
  match path.parent() {
    Some(parent_folder) => {
      match fs::create_dir_all(parent_folder) {
        Ok(_) => match fs::write(path, content) {
          Ok(_) => Ok(()),
          Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
      }
    }
    None => Err("Could not make directory".to_string()),
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn check_highlight_code_no_nums() {
    let left = highlight_code_no_nums(&[
      minijinja::Value::from("sierra foxtrot".to_string()),
      minijinja::Value::from("txt".to_string()),
    ]);
    let right =
      r#"<span class="text plain">sierra foxtrot</span>"#
        .to_string();
    assert_eq!(left, right);
  }
}
