use crate::section::list_item::*;
use crate::section::*;
use crate::section_attr::*;
use crate::span::*;
// use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn list_section_full<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
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
    let (source, children) = many0(|src| list_item_full(src, sections))
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfig;
    use pretty_assertions::assert_eq;
    #[test]
    fn solo_basic_list() {
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
                                        source_text: "alfa".to_string(),
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
        let right = list_section_full(source, &config.sections).unwrap();
        assert_eq!(left, right);
    }
}
