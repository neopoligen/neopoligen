use crate::section_attr_v42::flag::SectionFlagAttrV42;
use crate::section_attr_v42::key_value::SectionKeyValueAttrV42;
use crate::section_v42::*;
use crate::site_config::SiteConfigV42;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::rest;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BasicSectionV42 {
    pub attrs: Vec<SectionKeyValueAttrV42>,
    pub children: Vec<SectionV42>,
    pub flags: Vec<SectionFlagAttrV42>,
    pub kind: String,
}

pub fn basic_section_full_v42<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _nest_level: usize,
    _list_level: usize,
    _checklist_level: usize,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let _dbg_header = "BASIC_SECTION_START";
    let (source, _) = pair(tag("--"), space1).context("").parse(source)?;
    let (source, kind) = tag("title").context("").parse(source)?;
    let (source, _) = rest.context("").parse(source)?;
    let section = SectionV42::Basic(BasicSectionV42 {
        attrs: vec![],
        children: vec![],
        kind: kind.to_string(),
        flags: vec![],
    });
    Ok((source, section))
}

#[cfg(test)]
mod test {
    use super::*;
    use include_dir::{include_dir, Dir};
    use pretty_assertions::assert_eq;

    const SAMPLE_DIR: Dir = include_dir!("src/section_v42/samples/basic/valid");

    #[test]
    fn solo_pagedata_with_extra_data() {
        let site_config = SiteConfigV42::mock1_basic();
        let source = SAMPLE_DIR
            .get_file("001-single-block.neo")
            .unwrap()
            .contents_utf8()
            .unwrap()
            .to_string();
        let left = (
            "",
            SectionV42::Basic(BasicSectionV42 {
                attrs: vec![],
                children: vec![],
                kind: "title".to_string(),
                flags: vec![],
            }),
        );
        let right = basic_section_full_v42(&source, &site_config, 0, 0, 0).unwrap();
        assert_eq!(left, right);
    }
}
