use crate::helpers::dump_parser_if_debug;
use crate::section::block::*;
use crate::section::*;
use crate::section_attr::*;
// use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn basic_section_end<'a>(
    source: &'a str,
    key: &'a str,
    _nest_level: usize,
    list_level: usize,
    _checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let dbg_header = "_____ BASIC_SECTION_END _____";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    // let (source, _) = multispace0.context("").parse(source)?;
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, _) = tag("/").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, closing) = tag(key).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = multispace0.context("").parse(source)?;
    // let (source, _) = structure_empty_until_newline_or_eof
    //     .context("")
    //     .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = if list_level > 0 {
        many0(|src| block_of_list_item_content_v42(src))
            .context("")
            .parse(source)?
    // } else if nest_level > 0 {
    //     many0(alt((
    //         |src| block_of_main_content_v42(src),
    //         // basic_section_end(source, key, nest_level, list_level, checklist_level, debug),
    //     )))
    //     .context("")
    //     .parse(source)?
    } else {
        many0(|src| block_of_main_content_v42(src))
            .context("")
            .parse(source)?
    };
    let section = Section {
        attr_list: vec![],
        flag_list: vec![],
        attrs,
        bounds: SectionBounds::End,
        kind: SectionKind::End {
            children,
            closing: closing.to_string(),
        },
        r#type: "closing".to_string(),
    };
    Ok((source, section))
}

pub fn basic_section_full<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let dbg_header = "_____ BASIC_SECTION_FULL _____";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, r#type) = (|src| tag_finder_full(src, &sections.basic))
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    // let (source, attrs) = many0(section_attr).context("").parse(source)?;
    // let (source, _) = structure_empty_until_newline_or_eof
    //     .context("")
    //     .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| block_of_main_content_v42(src))
        .context("")
        .parse(source)?;
    let section = Section {
        attr_list: vec![],
        flag_list: vec![],
        attrs,
        bounds: SectionBounds::Full,
        kind: SectionKind::Basic { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn basic_section_start<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    mut nest_level: usize,
    list_level: usize,
    checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    nest_level += 1;
    let dbg_header = "_____ BASIC_SECTION_START _____";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, r#type) = (|src| tag_finder_start(src, &sections.basic))
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, mut children) = many0(alt((
        |src| block_of_main_content_v42(src),
        |src| {
            start_or_full_section(
                src,
                &sections,
                nest_level,
                list_level,
                checklist_level,
                debug,
            )
        },
    )))
    .context("")
    .parse(source)?;
    let (source, end_section) = basic_section_end(
        source,
        r#type,
        nest_level,
        list_level,
        checklist_level,
        debug,
    )?;
    children.push(end_section);
    let section = Section {
        attr_list: vec![],
        flag_list: vec![],
        attrs,
        bounds: SectionBounds::Start,
        kind: SectionKind::Basic { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    // TODO: put these tests back in place
    // #[case(
    //     "stop at next section",
    //     "-- div\n\nalfa bravo\n\n-- div",
    //     "-- div",
    //     "full",
    //     "",
    //     true
    // )]
    // #[case(
    //     "check and attribute",
    //     "-- div\n-- id: charlie\n\nalfa bravo\n\n-- div",
    //     "-- div",
    //     "full",
    //     "",
    //     true
    // )]
    // #[case(
    //     "test empty section with an attribute",
    //     "-- div\n-- id: charlie\n\n\n-- div",
    //     "-- div",
    //     "full",
    //     "",
    //     true
    // )]

    #[rstest]
    #[case(
        "basic section full test",
        "-- div\n\nalfa bravo",
        "",
        "full",
        "",
        true
    )]
    #[case(
        "test empty section at eof",
        "-- div\n-- id: charlie",
        "",
        "full",
        "",
        true
    )]
    #[ignore]
    fn basic_fixture(
        #[case] _description: &str,
        #[case] source: &str,
        #[case] remainder: &str,
        #[case] bounds: &str,
        #[case] _end_tag: &str,
        #[case] should_pass: bool,
    ) {
        let config = SiteConfig::mock1_basic();
        if should_pass {
            if bounds == "full" {
                assert_eq!(
                    remainder,
                    basic_section_full(source, &config.sections, 0, 0, 0, false)
                        .unwrap()
                        .0
                );
            }
        }
    }

    // #[rstest]
    // #[case("-- /css\n\n- alfa", "css", "line starts with hyphen")]
    // fn run_test(#[case] input: &str, #[case] end: &str, #[case] _description: &str) {
    //     let right = basic_section_end(input, end).unwrap().0;
    //     assert_eq!("", right);
    // }
    #[test]
    #[ignore]
    fn basic_full_test() {
        let source = "-- title\n\nHello World";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
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
                                parsed_text: "Hello World".to_string(),
                            }],
                        },
                        r#type: "block-of-text".to_string(),
                    }],
                },
                r#type: "title".to_string(),
            },
        );
        let right = basic_section_full(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn basic_start_text_with_dash_not_in_list() {
        let source = "-- p\n\n- Hello World";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
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
                                parsed_text: "- Hello World".to_string(),
                            }],
                        },
                        r#type: "block-of-text".to_string(),
                    }],
                },
                r#type: "p".to_string(),
            },
        );
        let right = basic_section_full(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn basic_full_test_checking_name_collision() {
        let source = "-- preview\n\nHello World";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
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
                                parsed_text: "Hello World".to_string(),
                            }],
                        },
                        r#type: "block-of-text".to_string(),
                    }],
                },
                r#type: "preview".to_string(),
            },
        );
        let right = basic_section_full(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn basic_start_end_test_checking_name_collision() {
        let source = "-- previews/\n\nHello World\n\n-- /previews";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attr_list: vec![],
                flag_list: vec![],
                attrs: vec![],
                bounds: SectionBounds::Start,
                kind: SectionKind::Basic {
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
                                    parsed_text: "Hello World".to_string(),
                                }],
                            },
                            r#type: "block-of-text".to_string(),
                        },
                        Section {
                            attr_list: vec![],
                            flag_list: vec![],
                            attrs: vec![],
                            bounds: SectionBounds::End,
                            kind: SectionKind::End {
                                children: vec![],
                                closing: "previews".to_string(),
                            },
                            r#type: "closing".to_string(),
                        },
                    ],
                },
                r#type: "previews".to_string(),
            },
        );
        let right = basic_section_start(source, &config.sections, 0, 0, 0, false);
        // dbg!(&right);
        assert_eq!(left, right.unwrap());
    }

    #[test]
    #[ignore]
    fn basic_with_code_shorthand() {
        let source = "-- title\n\n``code shorthand``";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
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
                                kind: SpanKind::CodeShorthand { children: vec![] },
                                parsed_text: "code shorthand".to_string(),
                            }],
                        },
                        r#type: "block-of-text".to_string(),
                    }],
                },
                r#type: "title".to_string(),
            },
        );
        let right = basic_section_full(source, &config.sections, 0, 0, 0, false).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn ending_without_nesting_and_no_content() {
        let source = "-- /p";
        let left = 0;
        let section = basic_section_end(source, "p", 0, 0, 0, false).unwrap().1;
        let right = match section.kind {
            SectionKind::Basic { children, .. } => children.len(),
            _ => 0,
        };
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn ending_without_nesting_and_has_content() {
        let source = "-- /p\n\nthis is\n\nsomethig";
        let left = 2;
        let section = basic_section_end(source, "p", 0, 0, 0, false).unwrap().1;
        // dbg!(&section);
        let right = match section.kind {
            SectionKind::End { children, .. } => children.len(),
            _ => 0,
        };
        assert_eq!(left, right);
    }

    // TODO: Check with proper list_nesting value
    // #[test]
    // fn ending_with_nesting() {
    //     let source = "-- /p\n\nmore stuff";
    //     let left = "more stuff".to_string();
    //     let right = basic_section_end(source, "p", 1).unwrap().0;
    //     assert_eq!(left, right);
    // }

    //
}
