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
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, children) = many1(|src| block_of_list_content(src))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Section {
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
                attrs: vec![],
                bounds: SectionBounds::Full,
                kind: SectionKind::ListItem {
                    children: {
                        vec![Section {
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
                        }]
                    },
                },
                r#type: "list-item".to_string(),
            },
        );
        let right = list_item_full(source, &config.sections).unwrap();
        assert_eq!(left, right);
    }
}
