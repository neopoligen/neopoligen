pub mod inits;
pub mod mocks;

use crate::neo_error::NeoError;
use crate::section_v42::SectionV42;
use crate::span_v42::plaintext::PlainTextSpan;
use crate::span_v42::SpanV42;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom::IResult;
use nom_supreme::error::ErrorTree;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageTitleV42 {
    spans: Vec<SpanV42>,
    text: String,
}

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

    // TODO: Deprecate in favor of the field
    pub fn html_file_system_relative_path(&self, base_path: &PathBuf) -> Result<PathBuf, NeoError> {
        if let Ok(p) = self.get_pagedata(&[Value::from("path")]) {
            if p != "" {
                let new_base = p.strip_prefix("/").unwrap();
                Ok(PathBuf::from(new_base))
            } else {
                Ok(base_path
                    .clone()
                    .join(self.id.clone().unwrap())
                    .join("index.html"))
            }
        } else {
            Ok(base_path
                .clone()
                .join(self.id.clone().unwrap())
                .join("index.html"))
        }
    }

    // // TODO: Deprecate in favor of top level .id (field not method)
    // pub fn tmp_id(&self) -> Result<String, Error> {
    //     Ok(self.id.as_ref().unwrap().to_string())
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
            Some(attrs) => {
                //dbg!(&attrs);
                match attrs.get("title") {
                    Some(attr) => Some(Ok(Value::from_serialize(PageTitleV42 {
                        spans: attr.spans.clone(),
                        text: attr.text.to_string(),
                    }))),
                    None => None,
                }
            }
            None => None,
        } {
            page_title
        } else if let Some(id) = &self.id.clone() {
            Ok(Value::from_serialize(PageTitleV42 {
                spans: vec![SpanV42::PlainText(PlainTextSpan { text: id.clone() })],
                text: id.to_string(),
            }))
        } else {
            Ok(Value::from("xTODO_PAGE_TITLE"))
        }
    }
}

// pub fn get_id_from_invalid_page(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
//     Ok((source, "no id"))
// }

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
    fn solo_title_from_pagedata_id() {
        let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
        let left = Value::from_serialize(PageTitleV42 {
            spans: vec![SpanV42::PlainText(PlainTextSpan {
                text: "001-pagedata-only.neo".to_string(),
            })],
            text: "001-pagedata-only.neo".to_string(),
        });
        let right = page.title().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_title_from_pagedata_title() {
        let page = PageV42::test_from_sample_file("005-title-in-pagedata.neo");
        let left = Value::from_serialize(PageTitleV42 {
            spans: vec![SpanV42::PlainText(PlainTextSpan {
                text: "Title In Pagedata".to_string(),
            })],
            text: "Title In Pagedata".to_string(),
        });
        let right = page.title().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_title_from_invalid_page() {
        let page = PageV42::test_from_sample_file("006-unclosed-start-section.neo");
        let left = Value::from_serialize(PageTitleV42 {
            spans: vec![SpanV42::PlainText(PlainTextSpan {
                text: "006-unclosed-start-section.neo".to_string(),
            })],
            text: "006-unclosed-start-section.neo".to_string(),
        });
        let right = page.title().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn title_in_pagedata() {
        let page = PageV42::test_from_sample_file("005-title-in-pagedata.neo");
        let left = Value::from_serialize(PageTitleV42 {
            spans: vec![SpanV42::PlainText(PlainTextSpan {
                text: "Title In Pagedata".to_string(),
            })],
            text: "Title In Pagedata".to_string(),
        });
        let right = page.title().unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn default_html_file_system_relative_path_from_id() {
    //     let page = PageV42::valid_sample_mock("001-pagedata-only.neo");
    //     let left = PathBuf::from("en/001-pagedata-only.neo/index.html");
    //     let right = page
    //         .html_file_system_relative_path(&PathBuf::from("en"))
    //         .unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn custom_path_in_pagedata() {
    //     let page = PageV42::valid_sample_mock("003-custom-path-in-pagedata.neo");
    //     let left = PathBuf::from("index.html");
    //     let right = page
    //         .html_file_system_relative_path(&PathBuf::from("en"))
    //         .unwrap();
    //     assert_eq!(left, right);
    // }

    // TODO: Redo so it calls the new build method
    #[test]
    #[ignore]
    fn unknown_start_end() {
        let page = PageV42::valid_sample_mock("004-unknown-start-end.neo");
        let left = "004-unknown-start-end.neo".to_string();
        let right = page.id.unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // pub fn solo_id_check_basic() {
    //     let page = PageV42::valid_sample_mock("001-pagedata-only.neo");
    //     let left = Value::from("001-pagedata-only".to_string());
    //     let right = page.id().unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // pub fn solo_check_2() {
    //     let page = PageV42::valid_sample_mock("002-basic-sample.neo");
    //     dbg!(page.ast().unwrap());
    //     // dbg!(page);
    // }

    // fn valid_samples() -> BTreeMap<String, String> {
    //     let mut samples = BTreeMap::new();
    //     samples.insert(
    //         "000-pagedata-only".to_string(),
    //         include_str!("samples/valid/000-pagedata-only.neo").to_string(),
    //     );
    //     samples.insert(
    //         "001-basic-sample".to_string(),
    //         include_str!("samples/valid/001-basic-sample.neo").to_string(),
    //     );
    //     samples
    // }
}
