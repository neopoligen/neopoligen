use crate::helpers::dump_parser_if_debug;
use crate::section::block::*;
use crate::section::*;
use crate::section_attr::*;
use crate::span::*;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn unknown_section_end<'a>(
    source: &'a str,
    key: &'a str,
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let dbg_header = "______________ UNKNOWN_SECTION_END";
    if debug {
        dbg!(format!("{}: {}", dbg_header, "1"));
        dbg!(&source);
    }
    let (source, _) = tag("-- ").context("").parse(source)?;
    if debug {
        dbg!(format!("{}: {}", dbg_header, "2"));
        dbg!(&source);
    }
    let (source, _) = tag("/").context("").parse(source)?;
    if debug {
        dbg!(format!("{}: {}", dbg_header, "3"));
        dbg!(&source);
    }
    let (source, r#type) = tag(key).context("").parse(source)?;
    if debug {
        dbg!(format!("{}: {}", dbg_header, "4"));
        dbg!(&source);
    }
    // let (source, _) = many0(pair(space0, newline)).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    // let (source, _) = structure_empty_until_newline_or_eof
    //     .context("")
    //     .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    // TODO: Watch for nested list stuff
    let (source, children) = many0(|src| block_of_main_content_v42(src))
        .context("")
        .parse(source)?;
    let section = Section {
        attr_list: vec![],
        flag_list: vec![],
        attrs,
        bounds: SectionBounds::End,
        kind: SectionKind::Unknown { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

// TODO: Make this use tag_finder_full

pub fn unknown_section_full<'a>(
    source: &'a str,
    _sections: &'a ConfigSections,
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let dbg_header = "______________ UNKNOWN_SECTION_FULL";
    dump_parser_if_debug(debug, dbg_header, &source, 1);
    let (source, _) = tag("-- ").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 2);
    let (source, r#type) = is_not(" /\n").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, &source, 3);
    // let (source, _) = structure_empty_until_newline_or_eof
    //     .context("")
    //     .parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    if debug {
        dbg!(format!("{}: {}", dbg_header, "4"));
        dbg!(&source);
    }
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
        kind: SectionKind::Unknown { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn unknown_section_start<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    nest_level: usize,
    list_level: usize,
    checklist_level: usize,
    debug: bool,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let dbg_header = "______________ UNKNOWN_SECTION_START";
    if debug {
        dbg!(format!("{}: {}", dbg_header, "1"));
        dbg!(&source);
    }
    let (source, _) = tag("-- ").context("").parse(source)?;
    if debug {
        dbg!(format!("{}: {}", dbg_header, "2"));
        dbg!(&source);
    }
    let (source, r#type) = is_not(" \n/").context("").parse(source)?;
    if debug {
        dbg!(format!("{}: {}", dbg_header, "3"));
        dbg!(&source);
    }
    let (source, _) = tag("/").context("").parse(source)?;
    if debug {
        dbg!(format!("{}: {}", dbg_header, "4"));
        dbg!(&source);
    }
    // let (source, _) = many0(pair(space0, newline)).context("").parse(source)?;
    // let (source, _) = structure_empty_until_newline_or_eof
    //     .context("")
    //     .parse(source)?;
    // let (source, r#type) = (|src| tag_finder_start(src, &sections.basic))
    //     .context("")
    //     .parse(source)?;
    // TODO: You can remove the lines below
    // the next time you see them
    // let (source, _) = tag("/").context("").parse(source)?;
    // let (source, _) = structure_empty_until_newline_or_eof
    //     .context("")
    //     .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    // dbg!("xxxxxxxxxxxxxxxxxx");
    // dbg!(&source);
    // let (source, _) = structure_empty_until_newline_or_eof
    //     .context("")
    //     .parse(source)?;
    // dbg!("----------------------");
    // dbg!(&source);
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, mut children) = many0(alt((
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
        |src| block_of_main_content_v42(src),
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
        kind: SectionKind::Unknown { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

// pub fn _unknown_section_start<'a>(
//     source: &'a str,
//     sections: &'a ConfigSections,
//     nest_level: usize,
//     list_level: usize,
// ) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
//     let (source, _) = tag("-- ").context("").parse(source)?;
//     let (source, r#type) = is_not(" /\n").context("").parse(source)?;
//     let (source, _) = tag("/").context("").parse(source)?;
//     let (source, _) = structure_empty_until_newline_or_eof
//         .context("")
//         .parse(source)?;
//     let (source, attrs) = many0(section_attr).context("").parse(source)?;
//     let (source, _) = structure_empty_until_newline_or_eof
//         .context("")
//         .parse(source)?;
//     let (source, _) = multispace0.context("").parse(source)?;
//     let (source, mut children) = many0(alt((
//         |src| block_of_anything(src),
//         |src| start_or_full_section(src, &sections, nest_level, list_level),
//     )))
//     .context("")
//     .parse(source)?;
//     let (source, end_section) = basic_section_end(source, r#type, nest_level, list_level)?;
//     children.push(end_section);
//     let section = Section {
//         attr_list: vec![],
//         flag_list: vec![],
//         attrs,
//         bounds: SectionBounds::Start,
//         kind: SectionKind::Unknown { children },
//         r#type: r#type.to_string(),
//     };
//     Ok((source, section))
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;
    //     use rstest::rstest;

    #[test]
    fn unknown_start_and_end_not_in_list() {
        let nest_level = 0;
        let list_level = 0;
        let checklist_level = 0;
        let debug = false;
        let source = "-- unknown-section/\n\nalfa bravo\n\n-- /unknown-section";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attr_list: vec![],
                flag_list: vec![],
                attrs: vec![],
                bounds: SectionBounds::Start,
                kind: SectionKind::Unknown {
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
                                    parsed_text: "alfa bravo".to_string(),
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
                                closing: "unknown-section".to_string(),
                            },
                            r#type: "closing".to_string(),
                        },
                    ],
                },
                r#type: "unknown-section".to_string(),
            },
        );
        let right = unknown_section_start(
            source,
            &config.sections,
            nest_level,
            list_level,
            checklist_level,
            debug,
        );
        // dbg!(&right);
        assert_eq!(left, right.unwrap());
    }

    #[test]
    fn basic_full_test() {
        let nest_level = 0;
        let list_level = 0;
        let checklist_level = 0;
        let debug = false;
        let source = "-- unknown-section\n\nHello World";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attr_list: vec![],
                flag_list: vec![],
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::Unknown {
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
                r#type: "unknown-section".to_string(),
            },
        );
        let right = unknown_section_full(
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

    #[test]
    fn ending_without_nesting_and_no_content() {
        let nest_level = 0;
        let list_level = 0;
        let checklist_level = 0;
        let debug = false;
        let source = "-- /asdf";
        let left = 0;
        let section = unknown_section_end(
            source,
            "asdf",
            nest_level,
            list_level,
            checklist_level,
            debug,
        )
        .unwrap()
        .1;
        let right = match section.kind {
            SectionKind::Unknown { children, .. } => children.len(),
            _ => 0,
        };
        assert_eq!(left, right);
    }

    #[test]
    fn ending_without_nesting_and_has_content() {
        let nest_level = 0;
        let list_level = 0;
        let checklist_level = 0;
        let debug = false;
        let source = "-- /asdf\n\nthis is\n\nsomethig";
        let left = 2;
        let section = unknown_section_end(
            source,
            "asdf",
            nest_level,
            list_level,
            checklist_level,
            debug,
        )
        .unwrap()
        .1;
        let right = match section.kind {
            SectionKind::Unknown { children, .. } => children.len(),
            _ => 0,
        };
        assert_eq!(left, right);
    }

    // TODO: Bring this back in with list nesting
    // #[test]
    // fn ending_with_nesting() {
    //     let source = "-- /asdf\n\nmore stuff";
    //     let left = "more stuff".to_string();
    //     let right = unknown_section_end(source, "asdf", 1).unwrap().0;
    //     assert_eq!(left, right);
    // }

    //
}
