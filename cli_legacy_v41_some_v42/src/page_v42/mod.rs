pub mod inits;
pub mod mocks;

use crate::neo_error::NeoError;
use crate::section_v42::SectionV42;
use crate::span_v42::plaintext::PlainTextSpan;
use crate::span_v42::SpanV42;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::ErrorKind;
use minijinja::Value;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV42 {
    pub id: Option<String>,
    pub raw_ast: Vec<SectionV42>,
    pub sections: Value,
    pub source: String,
    pub fs_path: PathBuf,
    pub html_output_path: Option<PathBuf>,
    pub error: Option<NeoError>,
}

impl PageV42 {
    pub fn ast(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.raw_ast))
    }

    // // TODO: Deprecate in favor of the field
    // pub fn html_file_system_relative_path(&self, base_path: &PathBuf) -> Result<PathBuf, NeoError> {
    //     if let Ok(p) = self.get_pagedata(&[Value::from("path")]) {
    //         if p != "" {
    //             let new_base = p.strip_prefix("/").unwrap();
    //             Ok(PathBuf::from(new_base))
    //         } else {
    //             Ok(base_path
    //                 .clone()
    //                 .join(self.id.clone().unwrap())
    //                 .join("index.html"))
    //         }
    //     } else {
    //         Ok(base_path
    //             .clone()
    //             .join(self.id.clone().unwrap())
    //             .join("index.html"))
    //     }
    // }

    pub fn get_pagedata(&self, args: &[Value]) -> Result<String, Error> {
        if args.len() == 0 {
            Err(Error::new(
                minijinja::ErrorKind::MissingArgument,
                "get_pagedata requires an argument with an item to search for",
            ))
        } else if args.len() == 1 {
            match &self
                .raw_ast
                .iter()
                .filter_map(|section| match section {
                    SectionV42::Json5(s) => Some(s),
                    _ => None,
                })
                .find_map(|section| match section.kind.as_str() {
                    "pagedata" => Some(&section.attrs),
                    _ => None,
                }) {
                Some(x) => match x.get(&args[0].to_string()) {
                    Some(v) => Ok(v.text.clone()),
                    None => Err(Error::new(
                        minijinja::ErrorKind::InvalidOperation,
                        format!("Could not find {} in pagedata", &args[0]),
                    )),
                },
                None => Err(Error::new(
                    minijinja::ErrorKind::InvalidOperation,
                    format!("Could not find {} in pagedata", &args[0]),
                )),
            }
        } else {
            Err(Error::new(
                minijinja::ErrorKind::TooManyArguments,
                "get_pagedata take only one argument for things to look for but more than one was passed in",
            ))
        }
    }

    pub fn make_value_from_string(&self, input: Result<String, Error>) -> Result<Value, Error> {
        match input {
            Ok(v) => Ok(Value::from(v)),
            Err(e) => Err(e),
        }
    }

    pub fn template_list(&self) -> Vec<String> {
        vec!["page-templates/post/done.neoj".to_string()]
    }

    pub fn title(&self) -> Result<Value, Error> {
        // ORDER:
        // pagedata--title
        // -- title section first block
        // --section--title attr on first block if it has one
        // first number of n words from first block
        // pagedata--id
        if let Some(page_title) = match &self
            .raw_ast
            .iter()
            .filter_map(|section| match section {
                SectionV42::Json5(s) => Some(s),
                _ => None,
            })
            .find_map(|section| match section.kind.as_str() {
                "pagedata" => {
                    //dbg!(&section.attrs);
                    Some(&section.attrs)
                }
                _ => None,
            }) {
            Some(attrs) => match attrs.get("title") {
                Some(attr) => Some(Ok(Value::from_serialize(attr.spans.clone()))),
                None => None,
            },
            None => None,
        } {
            page_title
        } else if let Some(spans) = &self.raw_ast.iter().find_map(|section| match section {
            SectionV42::Basic(sec) => match sec.kind.as_str() {
                "title" => sec.children.iter().find_map(|child| match child {
                    SectionV42::Block(c) => match c.kind.as_str() {
                        "paragraph" => Some(c.spans.clone()),
                        _ => None,
                    },
                    _ => None,
                }),
                _ => None,
            },
            _ => None,
        }) {
            //dbg!(spans);
            Ok(Value::from_serialize(spans))
        } else if let Some(id) = &self.id.clone() {
            Ok(Value::from_serialize(vec![SpanV42::PlainText(
                PlainTextSpan { text: id.clone() },
            )]))
        } else {
            Ok(Value::from("[NEO_TODO_PAGE_TITLE]"))
        }
    }

    pub fn href(&self) -> Result<Value, Error> {
        if let Some(original_path) = self.html_output_path.clone() {
            let base_path = original_path
                .to_string_lossy()
                .trim_end_matches("index.html")
                .to_string();
            Ok(Value::from(format!("/{}", base_path)))
        } else {
            Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("could not get href: "),
            ))
        }
    }

    //
}

impl Object for PageV42 {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "error" => match &self.error {
                Some(e) => Some(Value::from_serialize(e.clone())),
                None => Some(Value::from("TODO: output specific error here")),
            },
            "id" => Some(Value::from(self.id.clone())),
            "sections" => Some(self.sections.clone()),
            "source" => Some(Value::from(self.source.clone())),
            _ => None,
        }
    }

    fn call_method(
        self: &Arc<PageV42>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            // "sections" => self.ast(),
            "get_pagedata" => self.make_value_from_string(self.get_pagedata(args)),
            // "sections" => self.sections(),
            "title" => self.title(),
            _ => Ok(Value::from("")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span_v42::{plaintext::PlainTextSpan, SpanV42};
    use pretty_assertions::assert_eq;

    #[test]
    fn id_basic_check() {
        let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
        let left = "001-pagedata-only.neo".to_string();
        let right = page.id.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn id_from_invalid_page() {
        let page = PageV42::test_from_sample_file("006-unclosed-start-section.neo");
        let left = "006-unclosed-start-section.neo".to_string();
        let right = page.id.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn title_from_pagedata_id() {
        let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
        let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
            text: "001-pagedata-only.neo".to_string(),
        })]);
        let right = page.title().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn title_from_pagedata_title() {
        let page = PageV42::test_from_sample_file("005-title-in-pagedata.neo");
        let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
            text: "Title In Pagedata".to_string(),
        })]);
        let right = page.title().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn title_from_invalid_page() {
        let page = PageV42::test_from_sample_file("006-unclosed-start-section.neo");
        let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
            text: "006-unclosed-start-section.neo".to_string(),
        })]);
        let right = page.title().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_title_from_title_section() {
        let page = PageV42::test_from_sample_file("002-basic-sample.neo");
        let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
            text: "Basic Sample".to_string(),
        })]);
        let right = page.title().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn default_html_file_system_relative_path_from_id() {
        let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
        let left = PathBuf::from("en/001-pagedata-only.neo/index.html");
        let right = page.html_output_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn custom_path_in_pagedata() {
        let page = PageV42::test_from_sample_file("003-custom-href-in-pagedata.neo");
        let left = PathBuf::from("index.html");
        let right = page.html_output_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn href_from_id_default() {
        let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
        let left = Value::from("/en/001-pagedata-only.neo/".to_string());
        let right = page.href().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn href_from_pagedata() {
        let page = PageV42::test_from_sample_file("003-custom-href-in-pagedata.neo");
        let left = Value::from("/".to_string());
        let right = page.href().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn href_for_non_index_html_file() {
        let page = PageV42::test_from_sample_file("009-non-index-custom-href.neo");
        let left = Value::from("/somewhere/custom.html".to_string());
        let right = page.href().unwrap();
        assert_eq!(left, right);
    }

    //
}
