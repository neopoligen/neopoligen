use crate::section::block::*;
use crate::section::*;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::opt;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn checklist_item_full<'a>(
    source: &'a str,
    _sections: &'a ConfigSections,
    _nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("[").context("").parse(source)?;
    let (source, checked_value) = opt(is_not("]")).context("").parse(source)?;
    let (source, _) = tag("]").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, children) = many1(|src| block_of_checklist_content(src))
        .context("")
        .parse(source)?;
    let (checked, checked_string) = match checked_value {
        Some(v) => (true, Some(v.to_string())),
        None => (false, None),
    };
    Ok((
        source,
        Section {
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::ChecklistItem {
                checked,
                checked_string,
                children,
            },
            r#type: "checklist-item".to_string(),
        },
    ))
}

pub fn checklist_item_start_end<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, children) = many1(alt((
        |src| start_or_full_section(src, sections, nest_level),
        |src| block_of_list_content(src),
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        Section {
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::ChecklistItem {
                checked: false,
                checked_string: None,
                children,
            },
            r#type: "checklist-item".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_checklist_item_not_checked() {
        let source = "[] alfa\n\n";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::ChecklistItem {
                    checked: false,
                    checked_string: None,
                    children: {
                        vec![Section {
                            attrs: vec![],
                            bounds: SectionBounds::Full,
                            kind: SectionKind::Block {
                                spans: vec![Span {
                                    attrs: vec![],
                                    kind: SpanKind::WordPart,
                                    parsed_text: "alfa".to_string(),
                                }],
                            },
                            r#type: "block-of-text".to_string(),
                        }]
                    },
                },
                r#type: "checklist-item".to_string(),
            },
        );
        let right = checklist_item_full(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn basic_checklist_item_is_checked() {
        let source = "[x] alfa\n\n";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::ChecklistItem {
                    checked: true,
                    checked_string: Some("x".to_string()),
                    children: {
                        vec![Section {
                            attrs: vec![],
                            bounds: SectionBounds::Full,
                            kind: SectionKind::Block {
                                spans: vec![Span {
                                    attrs: vec![],
                                    kind: SpanKind::WordPart,
                                    parsed_text: "alfa".to_string(),
                                }],
                            },
                            r#type: "block-of-text".to_string(),
                        }]
                    },
                },
                r#type: "checklist-item".to_string(),
            },
        );
        let right = checklist_item_full(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    //
}
