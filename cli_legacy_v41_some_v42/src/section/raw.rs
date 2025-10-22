use crate::helpers::dump_parser_if_debug;
use crate::section::block::*;
use crate::section::*;
use crate::section_attr::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn raw_section_end<'a>(
    source: &'a str,
    key: &'a str,
    _nest_level: usize,
    list_level: usize,
    _checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let dbg_header = "RAW_SECTION_END";
    dump_parser_if_debug(debug, dbg_header, &source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 2);
    let (source, _) = tag("/").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 3);
    let (source, r#type) = tag(key).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 4);
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 5);
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 6);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 7);
    let (source, children) = if list_level > 0 {
        many0(|src| block_of_list_item_content_v42(src))
            .context("")
            .parse(source)?
    } else {
        many0(|src| block_of_main_content_v42(src))
            .context("")
            .parse(source)?
    };
    dump_parser_if_debug(debug, dbg_header, &source, 8);

    // let (source, children) = if list_level > 0 {
    //     many0(|src| block_of_end_content_that_is_not_a_list_item(src))
    //         .context("")
    //         .parse(source)?
    // } else {
    //     many0(|src| block_of_end_content(src))
    //         .context("")
    //         .parse(source)?
    // };

    let section = Section {
        attr_list: vec![],
        flag_list: vec![],
        attrs,
        bounds: SectionBounds::End,
        kind: SectionKind::End {
            children,
            closing: r#type.to_string(),
        },
        r#type: "closing".to_string(),
    };
    Ok((source, section))
}

pub fn raw_section_full<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let dbg_header = "RAW_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, &source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 2);
    let (source, r#type) = (|src| tag_finder_full(src, &sections.raw))
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 3);
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 4);
    let (source, text) = alt((take_until("\n--"), rest)).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 5);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 6);
    let section = Section {
        attr_list: vec![],
        flag_list: vec![],
        attrs,
        bounds: SectionBounds::Full,
        kind: SectionKind::Raw {
            children: vec![],
            text: Some(text.trim_end().to_string()),
        },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn raw_section_start<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    mut nest_level: usize,
    list_level: usize,
    checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    nest_level += 1;
    let dbg_header = "RAW_SECTION_START";
    dump_parser_if_debug(debug, dbg_header, &source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 2);
    let (source, r#type) = (|src| tag_finder_start(src, &sections.raw))
        .context("")
        .parse(source)?;
    let end_key = format!("-- /{}", r#type);
    dump_parser_if_debug(debug, dbg_header, &source, 3);
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 4);
    let (source, _) = many0(pair(space0, newline)).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 5);
    let (source, text) = take_until(end_key.as_str()).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 6);
    let (source, end_section) = raw_section_end(
        source,
        r#type,
        nest_level,
        list_level,
        checklist_level,
        debug,
    )?;
    dump_parser_if_debug(debug, dbg_header, &source, 7);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 8);
    let mut children = vec![];
    children.push(end_section);
    let section = Section {
        attr_list: vec![],
        flag_list: vec![],
        attrs,
        bounds: SectionBounds::Start,
        kind: SectionKind::Raw {
            children,
            text: Some(text.trim_end().to_string()),
        },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;

    // TODO: DEPRECATED: basic_section_end is now used for everything
    // this can be delete the next time you see it.
    // #[test]
    // fn ending_without_nesting_and_no_content() {
    //     let source = "-- /code";
    //     let left = 0;
    //     let section = raw_section_end(source, "code", 0, 0).unwrap().1;
    //     let right = match section.kind {
    //         SectionKind::Raw { children, .. } => children.len(),
    //         _ => 0,
    //     };
    //     assert_eq!(left, right);
    // }

    // TODO: DEPRECATED: basic_section_end is now used for everything
    // this can be delete the next time you see it.
    // #[test]
    // fn ending_without_nesting_and_has_content() {
    //     let source = "-- /code\n\nthis is\n\nsomethig";
    //     let left = 2;
    //     let section = raw_section_end(source, "code", 0, 0).unwrap().1;
    //     let right = match section.kind {
    //         SectionKind::Raw { children, .. } => children.len(),
    //         _ => 0,
    //     };
    //     assert_eq!(left, right);
    // }

    // TODO: Bring back in with proper list_nesting check
    // #[test]
    // #[ignore]
    // fn ending_with_nesting() {
    //     let source = "-- /code\n\nmore stuff";
    //     let left = "more stuff".to_string();
    //     let right = raw_section_end(source, "code", 1).unwrap().0;
    //     assert_eq!(left, right);
    // }

    #[test]
    #[ignore]
    fn basic_start_end() {
        let source = "-- code/\n\nsome code\n\n-- /code\n\n";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
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
                    text: Some("some code".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn basic_start_end_with_attrs() {
        let source = "-- code/\n-- bash\n-- class: alfa\n\nsome code\n\n-- /code\n\n";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attr_list: vec![],
                flag_list: vec![],
                attrs: vec![
                    SectionAttr {
                        kind: SectionAttrKind::Flag {
                            flag: "bash".to_string(),
                        },
                    },
                    SectionAttr {
                        kind: SectionAttrKind::KeyValueSpans {
                            key: "class".to_string(),
                            spans: vec![Span {
                                attrs: vec![],
                                kind: SpanKind::PlainText,
                                parsed_text: "alfa".to_string(),
                            }],
                        },
                    },
                ],
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
                    text: Some("some code".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn somewith_with_dashes_in_it() {
        let source = "-- code/\n\nalfa -- not here\n\n-- /code\n\n";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
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
                    text: Some("alfa -- not here".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn start_end_with_content_after_end() {
        let source = "-- code/\n\nalfa\n\n-- /code\n\nbravo\n\ncharlie\n\n-- p";
        let config = SiteConfig::mock1_basic();
        let left = (
            "-- p",
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
                            closing: "code".to_string(),
                            children: vec![
                                Section {
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
                                },
                                Section {
                                    attr_list: vec![],
                                    flag_list: vec![],
                                    attrs: vec![],
                                    bounds: SectionBounds::Full,
                                    kind: SectionKind::Block {
                                        spans: vec![Span {
                                            attrs: vec![],
                                            kind: SpanKind::PlainText,
                                            parsed_text: "charlie".to_string(),
                                        }],
                                    },
                                    r#type: "block-of-text".to_string(),
                                },
                            ],
                        },
                        r#type: "closing".to_string(),
                    }],
                    text: Some("alfa".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn start_end_with_list_item_after_when_in_list() {
        let source = "-- code/\n\nalfa\n\n-- /code\n\n- bravo";
        let config = SiteConfig::mock1_basic();
        let left = (
            "- bravo",
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
                    text: Some("alfa".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections, 1, 1, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn start_end_with_list_item_after_when_not_in_a_list() {
        let nest_level = 0;
        let list_level = 0;
        let checklist_level = 0;
        let debug = false;
        let source = "-- code/\n\nalfa\n\n-- /code\n\n- bravo";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
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
                            children: vec![Section {
                                attr_list: vec![],
                                attrs: vec![],
                                bounds: SectionBounds::Full,
                                flag_list: vec![],
                                kind: SectionKind::Block {
                                    spans: vec![Span {
                                        attrs: vec![],
                                        kind: SpanKind::PlainText,
                                        parsed_text: "- bravo".to_string(),
                                    }],
                                },
                                r#type: "block-of-text".to_string(),
                            }],
                            closing: "code".to_string(),
                        },
                        r#type: "closing".to_string(),
                    }],
                    text: Some("alfa".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(
            source,
            &config.sections,
            nest_level,
            list_level,
            checklist_level,
            debug,
        )
        .unwrap();
        assert_eq!(left, right);
    }

    //
}
