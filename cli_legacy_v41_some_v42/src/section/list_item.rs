use crate::section::block::*;
use crate::section::*;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn list_item_full<'a>(
    source: &'a str,
    _sections: &'a ConfigSections,
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
    _debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    //let (source, children) = many1(|src| block_of_list_content(src))
    let (source, children) = many1(|src| block_of_list_item_content_v42(src))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::ListItem { children },
            r#type: "list-item".to_string(),
        },
    ))
}

pub fn list_item_start_end<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    nest_level: usize,
    list_level: usize,
    checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, children) = many1(alt((
        |src| start_or_full_section(src, sections, nest_level, list_level, checklist_level, debug),
        // |src| block_of_list_content_v42(src),
        //|src| block_of_end_content_that_is_not_a_list_item(src),
        |src| block_of_list_item_content_v42(src),
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::ListItem { children },
            r#type: "list-item".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_list_item() {
        let source = "- alfa\n\n";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attr_list: vec![],
                flag_list: vec![],
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::ListItem {
                    children: {
                        vec![Section {
                            attr_list: vec![],
                            flag_list: vec![],
                            attrs: vec![],
                            bounds: SectionBounds::Full,
                            kind: SectionKind::Block {
                                spans: vec![Span {
                                    attrs: vec![],
                                    kind: SpanKind::PlainText,
                                    parsed_text: "alfa".to_string(),
                                }],
                            },
                            r#type: "block-of-text".to_string(),
                        }]
                    },
                },
                r#type: "list-item".to_string(),
            },
        );
        let right = list_item_full(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn start_end_list_items() {
        let source = "- alfa\n\n-- p\n\nbravo";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attr_list: vec![],
                flag_list: vec![],
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::ListItem {
                    children: {
                        vec![
                            Section {
                                attr_list: vec![],
                                flag_list: vec![],
                                attrs: vec![],
                                bounds: SectionBounds::Full,
                                kind: SectionKind::Block {
                                    spans: vec![Span {
                                        attrs: vec![],
                                        kind: SpanKind::PlainText,
                                        parsed_text: "alfa".to_string(),
                                    }],
                                },
                                r#type: "block-of-text".to_string(),
                            },
                            Section {
                                attr_list: vec![],
                                flag_list: vec![],
                                attrs: vec![],
                                bounds: SectionBounds::Full,
                                kind: SectionKind::Basic {
                                    children: vec![Section {
                                        attr_list: vec![],
                                        flag_list: vec![],
                                        attrs: vec![],
                                        bounds: SectionBounds::Full,
                                        kind: SectionKind::Block {
                                            spans: vec![Span {
                                                attrs: vec![],
                                                kind: SpanKind::PlainText,
                                                parsed_text: "bravo".to_string(),
                                            }],
                                        },
                                        r#type: "block-of-text".to_string(),
                                    }],
                                },
                                r#type: "p".to_string(),
                            },
                        ]
                    },
                },
                r#type: "list-item".to_string(),
            },
        );
        let right = list_item_start_end(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn start_end_list_items_with_code_section_in_a_list() {
        let source = "- alfa\n\n-- code/\n\nbravo\n\n-- /code\n\n- charlie\n\n-- /list";
        let config = SiteConfig::mock1_basic();
        let left = (
            "- charlie\n\n-- /list",
            Section {
                attr_list: vec![],
                flag_list: vec![],
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::ListItem {
                    children: {
                        vec![
                            Section {
                                attr_list: vec![],
                                flag_list: vec![],
                                attrs: vec![],
                                bounds: SectionBounds::Full,
                                kind: SectionKind::Block {
                                    spans: vec![Span {
                                        attrs: vec![],
                                        kind: SpanKind::PlainText,
                                        parsed_text: "alfa".to_string(),
                                    }],
                                },
                                r#type: "block-of-text".to_string(),
                            },
                            Section {
                                attr_list: vec![],
                                flag_list: vec![],
                                attrs: vec![],
                                bounds: SectionBounds::Start,
                                kind: SectionKind::Raw {
                                    children: vec![Section {
                                        attr_list: vec![],
                                        flag_list: vec![],
                                        attrs: vec![],
                                        bounds: SectionBounds::End,
                                        kind: SectionKind::End {
                                            children: vec![],
                                            closing: "code".to_string(),
                                        },
                                        r#type: "closing".to_string(),
                                    }],
                                    text: Some("bravo".to_string()),
                                },
                                r#type: "code".to_string(),
                            },
                        ]
                    },
                },
                r#type: "list-item".to_string(),
            },
        );
        let right = list_item_start_end(source, &config.sections, 1, 1, 0, false).unwrap();
        assert_eq!(left, right);
    }

    //
}
