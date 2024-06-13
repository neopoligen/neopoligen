use crate::section::block::*;
use crate::section::*;
use crate::section_attr::*;
use crate::span::*;
//use nom::branch::alt;
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
    let (source, children) = many0(|src| block_of_end_content(src))
        .context("")
        .parse(source)?;
    let section = Section {
        attrs,
        bounds: SectionBounds::End,
        kind: SectionKind::Unknown { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn unknown_section_full<'a>(
    source: &'a str,
    _sections: &'a ConfigSections,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = is_not(" /\n").context("").parse(source)?;
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
        kind: SectionKind::Unknown { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn unknown_section_start<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = is_not(" /\n").context("").parse(source)?;
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
        |src| start_or_full_section(src, &sections),
    )))
    .context("")
    .parse(source)?;
    let (source, end_section) = unknown_section_end(source, r#type)?;
    children.push(end_section);
    let section = Section {
        attrs,
        bounds: SectionBounds::Start,
        kind: SectionKind::Unknown { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;
    //     use rstest::rstest;

    #[test]
    fn solo_todo_unknow_start_and_end() {
        let source = "-- unknown-section/\n\nalfa bravo\n\n-- /unknown-section";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Start,
                kind: SectionKind::Unknown {
                    children: vec![
                        Section {
                            attrs: vec![],
                            bounds: SectionBounds::Full,
                            kind: SectionKind::Block {
                                spans: vec![Span {
                                    attrs: vec![],
                                    kind: SpanKind::WordPart,
                                    parsed_text: "alfa bravo".to_string(),
                                }],
                            },
                            r#type: "block-of-text".to_string(),
                        },
                        Section {
                            attrs: vec![],
                            bounds: SectionBounds::End,
                            kind: SectionKind::Unknown { children: vec![] },
                            r#type: "unknown-section".to_string(),
                        },
                    ],
                },
                r#type: "unknown-section".to_string(),
            },
        );
        let right = unknown_section_start(source, &config.sections).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn basic_full_test() {
        let source = "-- unknown-section\n\nHello World";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::Unknown {
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
                r#type: "unknown-section".to_string(),
            },
        );
        let right = unknown_section_full(source, &config.sections).unwrap();
        assert_eq!(left, right);
    }

    //
}
