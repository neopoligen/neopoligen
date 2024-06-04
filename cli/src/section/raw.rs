use crate::section::*;
use crate::section_attr::*;
use crate::site_config::SiteConfig;
use crate::span::*;
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

pub fn raw_section_full<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    _spans: &'a Vec<String>,
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
            text: Some(text.to_string()),
        },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn raw_section_start<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
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
    let (source, end_section) = basic_section_end(source, r#type)?;
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
    use pretty_assertions::assert_eq;
    #[test]
    fn basic_star_end() {
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
                        kind: SectionKind::Basic { children: vec![] },
                        r#type: "code".to_string(),
                    }],
                    text: Some("some code".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections).unwrap();
        assert_eq!(left, right);
    }

    #[test]
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
        let right = raw_section_start(source, &config.sections).unwrap();
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
                        kind: SectionKind::Basic { children: vec![] },
                        r#type: "code".to_string(),
                    }],
                    text: Some("alfa -- not here".to_string()),
                },
                r#type: "code".to_string(),
            },
        );
        let right = raw_section_start(source, &config.sections).unwrap();
        assert_eq!(left, right);
    }
}
