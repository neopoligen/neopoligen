use crate::section::*;
use crate::section_attr::*;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

// TODO: Make json5_section_start and json5_section_end

pub fn json5_section_full<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
    _nest_level: usize,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder_full(src, &sections.json5))
        .context("")
        .parse(source)?;
    // let (source, _) = structure_empty_until_newline_or_eof
    //     .context("")
    //     .parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, _text) = alt((take_until("\n--"), rest, eof))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attr_list: vec![],
            flag_list: vec![],
            attrs,
            bounds: SectionBounds::Full,
            kind: SectionKind::Json5 {
                children: vec![],
                // TODO: actually code this instead of using hard coded value
                data: serde_json5::from_str("{}").unwrap(),
            },
            r#type: r#type.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::site_config::SiteConfigV41;
    // use pretty_assertions::assert_eq;

    #[test]
    fn json5_basic_test() {
        let source = "-- pagedata\n-- id: abcd1234";
        let config = SiteConfigV41::mock1_basic();
        let response = json5_section_full(source, &config.sections, 0);
        assert!(response.is_ok());
    }
}
