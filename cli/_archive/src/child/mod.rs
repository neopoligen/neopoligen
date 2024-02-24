use crate::block::block;
use crate::config::Config;
use crate::json_plugin::JsonPlugin;
use crate::list::list;
use crate::list::List;
use crate::section::Section;
use crate::section::*;
use crate::span::*;
use crate::text_plugin::TextPlugin;
use nom::branch::alt;
use nom::combinator::eof;
use nom::combinator::not;
use nom::IResult;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(content = "content", rename_all = "lowercase", tag = "type")]
pub enum Child {
    Block(Vec<Span>),
    JsonPlugin(JsonPlugin),
    List(List),
    Section(Section),
    TextPlugin(TextPlugin),
}

pub fn child<'a>(source: &'a str, config: &'a Config) -> IResult<&'a str, Child> {
    let (source, _) = not(eof)(source)?;
    let (source, response) = alt((
        |src| block(src, config),
        |src| section(src, config),
        |src| list(src, config),
    ))(source)?;
    Ok((source, response))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::section_category::SectionCategory;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    #[test]
    // #[ignore]
    fn child_full_basic() {
        let source = "-- title\n\nTitle Sierra\n\n--p";
        let config = Config::mock_basic_config();
        let left = Ok((
            "--p",
            Child::Section(Section {
                key_value_attributes: BTreeMap::new(),
                flag_attributes: BTreeSet::new(),
                bounds: "full".to_string(),
                category: SectionCategory::StandardSectionFull {
                    containers: vec![Child::Block(vec![
                        Span::Word {
                            text: "Title".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                        Span::Space {
                            text: " ".to_string(),
                            template: "spans/space.jinja".to_string(),
                        },
                        Span::Word {
                            text: "Sierra".to_string(),
                            template: "spans/word.jinja".to_string(),
                        },
                    ])],
                },
                template: "default".to_string(),
                r#type: "title".to_string(),
                source: "-- title\n\nTitle Sierra".to_string(),
            }),
        ));
        let right = child(source, &config);
        assert_eq!(left, right);
    }
}
