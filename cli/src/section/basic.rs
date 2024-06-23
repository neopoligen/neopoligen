use crate::section::block::*;
use crate::section::*;
use crate::section_attr::*;
use crate::span::*;
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
    nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, r#type) = tag(key).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = if nest_level == 0 {
        many0(|src| block_of_end_content(src))
            .context("")
            .parse(source)?
    } else {
        (source, vec![])
    };
    let section = Section {
        attrs,
        bounds: SectionBounds::End,
        kind: SectionKind::Basic { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn basic_section_full<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    _nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.basic))
        .context("")
        .parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| block_of_anything(src))
        .context("")
        .parse(source)?;
    let section = Section {
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
    nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.basic))
        .context("")
        .parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, mut children) = many0(alt((
        |src| block_of_anything(src),
        |src| start_or_full_section(src, &sections, nest_level),
    )))
    .context("")
    .parse(source)?;
    let (source, end_section) = basic_section_end(source, r#type, nest_level)?;
    children.push(end_section);
    let section = Section {
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
        "stop at next section",
        "-- div\n\nalfa bravo\n\n-- div",
        "-- div",
        "full",
        "",
        true
    )]
    #[case(
        "check and attribute",
        "-- div\n-- id: charlie\n\nalfa bravo\n\n-- div",
        "-- div",
        "full",
        "",
        true
    )]
    #[case(
        "test empty section with an attribute",
        "-- div\n-- id: charlie\n\n\n-- div",
        "-- div",
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
                    basic_section_full(source, &config.sections, 0).unwrap().0
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
    fn basic_full_test() {
        let source = "-- title\n\nHello World";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::Basic {
                    children: vec![Section {
                        attrs: vec![],
                        bounds: SectionBounds::Full,
                        kind: SectionKind::Block {
                            spans: vec![Span {
                                attrs: vec![],
                                kind: SpanKind::WordPart,
                                parsed_text: "Hello World".to_string(),
                            }],
                        },
                        r#type: "block-of-text".to_string(),
                    }],
                },
                r#type: "title".to_string(),
            },
        );
        let right = basic_section_full(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn basic_with_code_shorthand() {
        let source = "-- title\n\n``code shorthand``";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::Basic {
                    children: vec![Section {
                        attrs: vec![],
                        bounds: SectionBounds::Full,
                        kind: SectionKind::Block {
                            spans: vec![Span {
                                attrs: vec![],
                                kind: SpanKind::CodeShorthand,
                                parsed_text: "code shorthand".to_string(),
                            }],
                        },
                        r#type: "block-of-text".to_string(),
                    }],
                },
                r#type: "title".to_string(),
            },
        );
        let right = basic_section_full(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn ending_without_nesting_and_no_content() {
        let source = "-- /p";
        let left = 0;
        let section = basic_section_end(source, "p", 0).unwrap().1;
        let right = match section.kind {
            SectionKind::Basic { children, .. } => children.len(),
            _ => 0,
        };
        assert_eq!(left, right);
    }

    #[test]
    fn ending_without_nesting_and_has_content() {
        let source = "-- /p\n\nthis is\n\nsomethig";
        let left = 2;
        let section = basic_section_end(source, "p", 0).unwrap().1;
        let right = match section.kind {
            SectionKind::Basic { children, .. } => children.len(),
            _ => 0,
        };
        assert_eq!(left, right);
    }

    #[test]
    fn solo_ending_with_nesting() {
        let source = "-- /p\n\nmore stuff";
        let left = "more stuff".to_string();
        let right = basic_section_end(source, "p", 1).unwrap().0;
        assert_eq!(left, right);
    }

    //
}
