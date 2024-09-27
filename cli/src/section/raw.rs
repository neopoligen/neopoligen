use crate::section::block::*;
use crate::section::*;
use crate::section_attr::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn raw_section_end<'a>(
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
        many0(|src| block_of_anything(src))
            .context("")
            .parse(source)?
    };
    let section = Section {
        attrs,
        bounds: SectionBounds::End,
        kind: SectionKind::Raw {
            children,
            text: None,
        },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn raw_section_full<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    _nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.raw))
        .context("")
        .parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, text) = alt((take_until("\n--"), rest)).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let section = Section {
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
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    nest_level += 1;
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.raw))
        .context("")
        .parse(source)?;
    let end_key = format!("-- /{}", r#type);
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, text) = take_until(end_key.as_str()).context("").parse(source)?;
    let (source, end_section) = raw_section_end(source, r#type, nest_level)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let mut children = vec![];
    children.push(end_section);
    let section = Section {
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

    #[test]
    fn ending_without_nesting_and_no_content() {
        let source = "-- /code";
        let left = 0;
        let section = raw_section_end(source, "code", 0).unwrap().1;
        let right = match section.kind {
            SectionKind::Raw { children, .. } => children.len(),
            _ => 0,
        };
        assert_eq!(left, right);
    }

    #[test]
    fn ending_without_nesting_and_has_content() {
        let source = "-- /code\n\nthis is\n\nsomethig";
        let left = 2;
        let section = raw_section_end(source, "code", 0).unwrap().1;
        let right = match section.kind {
            SectionKind::Raw { children, .. } => children.len(),
            _ => 0,
        };
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn ending_with_nesting() {
        let source = "-- /code\n\nmore stuff";
        let left = "more stuff".to_string();
        let right = raw_section_end(source, "code", 1).unwrap().0;
        assert_eq!(left, right);
    }

    #[test]
    fn basic_start_end() {
        let source = "-- code/\n\nsome code\n\n-- /code\n\n";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Start,
                kind: SectionKind::Raw {
                    children: vec![Section {
                        attrs: vec![],
                        bounds: SectionBounds::End,
                        kind: SectionKind::Raw {
                            text: None,
                            children: vec![],
                        },
                        r#type: "code".to_string(),
                    }],
                    text: Some("some code".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn basic_star_end_with_attrs() {
        let source = "-- code/\n-- bash\n-- class: alfa\n\nsome code\n\n-- /code\n\n";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![
                    SectionAttr {
                        kind: SectionAttrKind::Flag {
                            flag: "bash".to_string(),
                        },
                    },
                    SectionAttr {
                        kind: SectionAttrKind::KeyValue {
                            key: "class".to_string(),
                            value: "alfa".to_string(),
                        },
                    },
                ],
                bounds: SectionBounds::Start,
                kind: SectionKind::Raw {
                    children: vec![Section {
                        attrs: vec![],
                        bounds: SectionBounds::End,
                        kind: SectionKind::Basic { children: vec![] },
                        r#type: "code".to_string(),
                    }],
                    text: Some("some code".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn somewith_with_dashes_in_it() {
        let source = "-- code/\n\nalfa -- not here\n\n-- /code\n\n";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Start,
                kind: SectionKind::Raw {
                    children: vec![Section {
                        attrs: vec![],
                        bounds: SectionBounds::End,
                        kind: SectionKind::Raw {
                            children: vec![],
                            text: None,
                        },
                        r#type: "code".to_string(),
                    }],
                    text: Some("alfa -- not here".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_start_end_with_content_after_end() {
        let source = "-- code/\n\nalfa\n\n-- /code\n\nbravo\n\ncharlie\n\n-- p";
        let config = SiteConfig::mock1_basic();
        let left = (
            "-- p",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Start,
                kind: SectionKind::Raw {
                    children: vec![Section {
                        attrs: vec![],
                        bounds: SectionBounds::End,
                        kind: SectionKind::Raw {
                            text: None,
                            children: vec![
                                Section {
                                    attrs: vec![],
                                    bounds: SectionBounds::Full,
                                    kind: SectionKind::Block {
                                        spans: vec![Span {
                                            attrs: vec![],
                                            kind: SpanKind::WordPart,
                                            parsed_text: "bravo".to_string(),
                                        }],
                                    },
                                    r#type: "block-of-text".to_string(),
                                },
                                Section {
                                    attrs: vec![],
                                    bounds: SectionBounds::Full,
                                    kind: SectionKind::Block {
                                        spans: vec![Span {
                                            attrs: vec![],
                                            kind: SpanKind::WordPart,
                                            parsed_text: "charlie".to_string(),
                                        }],
                                    },
                                    r#type: "block-of-text".to_string(),
                                },
                            ],
                        },
                        r#type: "code".to_string(),
                    }],
                    text: Some("alfa".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    //
}
