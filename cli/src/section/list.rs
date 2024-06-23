use crate::section::list_item::*;
use crate::section::*;
use crate::section_attr::*;
use crate::span::*;
// use nom::branch::alt;
use crate::section::block::block_of_end_content;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn list_section_end<'a>(
    source: &'a str,
    key: &'a str,
    _nest_level: usize,
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
        kind: SectionKind::List { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn list_section_full<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    mut nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    nest_level += 1;
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.list))
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
    let (source, children) = many1(|src| list_item_full(src, sections, nest_level))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Section {
            attrs,
            bounds: SectionBounds::Full,
            kind: SectionKind::List { children },
            r#type: r#type.to_string(),
        },
    ))
}

pub fn list_section_start<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    mut nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    nest_level += 1;
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.list))
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
    let (source, mut children) = many1(|src| list_item_start_end(src, sections, nest_level))
        .context("")
        .parse(source)?;
    let (source, end_section) = list_section_end(source, r#type, nest_level)?;
    children.push(end_section);
    Ok((
        source,
        Section {
            attrs,
            bounds: SectionBounds::Start,
            kind: SectionKind::List { children },
            r#type: r#type.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_list_full() {
        let source = "-- list\n\n- alfa";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::List {
                    children: vec![Section {
                        attrs: vec![],
                        bounds: SectionBounds::Full,
                        kind: SectionKind::ListItem {
                            children: vec![Section {
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
                            }],
                        },
                        r#type: "list-item".to_string(),
                    }],
                },
                r#type: "list".to_string(),
            },
        );
        let right = list_section_full(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn basic_start_list_test() {
        let source = "-- list/\n\n- alfa\n\n-- /list";
        let config = SiteConfig::mock1_basic();
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::Start,
                kind: SectionKind::List {
                    children: vec![
                        Section {
                            attrs: vec![],
                            bounds: SectionBounds::Full,
                            kind: SectionKind::ListItem {
                                children: vec![Section {
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
                                }],
                            },
                            r#type: "list-item".to_string(),
                        },
                        Section {
                            attrs: vec![],
                            bounds: SectionBounds::End,
                            kind: SectionKind::List { children: vec![] },
                            r#type: "list".to_string(),
                        },
                    ],
                },
                r#type: "list".to_string(),
            },
        );
        let right = list_section_start(source, &config.sections, 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn basic_end_list_test() {
        let source = "-- /list";
        let left = (
            "",
            Section {
                attrs: vec![],
                bounds: SectionBounds::End,
                kind: SectionKind::List { children: vec![] },
                r#type: "list".to_string(),
            },
        );
        let right = list_section_end(source, "list", 0).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn link_check() {
        let config = SiteConfig::mock1_basic();
        let source = "-- list\n\n- <<link|Main site link|https://daverupert.com/2021/10/html-with-superpowers/>>";
        let left = "";
        let right = list_section_full(source, &config.sections, 0).unwrap().0;
        assert_eq!(left, right);
    }

    //
}
