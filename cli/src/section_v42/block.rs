use super::helpers::make_template_list::make_template_list;
use super::SectionParent;
use crate::helpers::dump_parser_if_debug;
use crate::section_v42::SectionV42;
use crate::site_config::SiteConfigV42;
use crate::span_v42::named_span::named_span;
use crate::span_v42::shorthand_code::code_shorthand_span;
use crate::span_v42::shorthand_emphasis::emphasis_shorthand_span;
use crate::span_v42::shorthand_footnote::footnote_shorthand_span;
use crate::span_v42::shorthand_html::html_shorthand_span;
use crate::span_v42::shorthand_image::image_shorthand_span;
use crate::span_v42::shorthand_link::link_shorthand_span;
use crate::span_v42::shorthand_mark::mark_shorthand_span;
use crate::span_v42::shorthand_strikethrough::strikethrough_shorthand_span;
use crate::span_v42::shorthand_strong::strong_shorthand_span;
use crate::span_v42::shorthand_underline::underline_shorthand_span;
use crate::span_v42::string_parsers::span_of_escaped_character;
use crate::span_v42::string_parsers::span_of_plain_text_for_block_paragraph;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlockSectionV42 {
    // Reminder: there won't be children for a block, but having
    // the field keeps the API consistent which is most important
    // because it makes looping easier
    pub children: Vec<SectionV42>,
    pub kind: String,
    pub spans: Vec<SpanV42>,
    pub template_list: Vec<String>,
}

pub fn checklist_item_paragraph_block<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
    kind: &str,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "CHECKLIST_ITEM_PARAGRAPH_BLOCK";
    // dump_parser_if_debug(debug, dbg_header, source, 1);
    // let (source, _) = not(tag("[")).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, spans) = preceded(
        pair(not(tag("[")), not(tag("-"))),
        many1(alt((
            span_of_plain_text_for_block_paragraph,
            span_of_escaped_character,
            named_span,
            code_shorthand_span,
            emphasis_shorthand_span,
            footnote_shorthand_span,
            html_shorthand_span,
            image_shorthand_span,
            link_shorthand_span,
            mark_shorthand_span,
            strikethrough_shorthand_span,
            strong_shorthand_span,
            underline_shorthand_span,
        ))),
    )
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, _) = multispace0.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let kind_name = format!("{}-item-paragraph", kind);
    Ok((
        source,
        SectionV42::Block(BlockSectionV42 {
            children: vec![],
            kind: kind_name.clone(),
            spans,
            template_list: make_template_list("checklist-item-paragraph", &kind_name, "default"),
        }),
    ))
}

pub fn list_item_paragraph_block<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
    kind: &str,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "LIST_ITEM_PARAGRAPH_BLOCK";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, spans) = many1(alt((
        span_of_plain_text_for_block_paragraph,
        span_of_escaped_character,
        named_span,
        code_shorthand_span,
        emphasis_shorthand_span,
        footnote_shorthand_span,
        html_shorthand_span,
        image_shorthand_span,
        link_shorthand_span,
        mark_shorthand_span,
        strikethrough_shorthand_span,
        strong_shorthand_span,
        underline_shorthand_span,
    )))
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, _) = multispace0.context("").parse(source)?;
    let kind_tag = format!("{}-item-paragraph", kind);
    Ok((
        source,
        SectionV42::Block(BlockSectionV42 {
            children: vec![],
            kind: kind_tag.clone(),
            spans,
            template_list: make_template_list("list-item-paragraph", &kind_tag, "default"),
        }),
    ))
}

pub fn olist_item_paragraph_block<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
    kind: &str,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "OLIST_ITEM_PARAGRAPH_BLOCK";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = pair(not(tag("-")), not(terminated(digit1, tag(". "))))
        .context("")
        .parse(source)?;
    let (source, spans) = many1(alt((
        span_of_plain_text_for_block_paragraph,
        span_of_escaped_character,
        named_span,
        code_shorthand_span,
        emphasis_shorthand_span,
        footnote_shorthand_span,
        html_shorthand_span,
        image_shorthand_span,
        link_shorthand_span,
        mark_shorthand_span,
        strikethrough_shorthand_span,
        strong_shorthand_span,
        underline_shorthand_span,
    )))
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let kind_tag = format!("{}-item-paragraph", kind);
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        SectionV42::Block(BlockSectionV42 {
            children: vec![],
            kind: kind_tag.clone(),
            spans,
            template_list: make_template_list("olist-item-paragraph", &kind_tag, "default"),
        }),
    ))
}

pub fn paragraph_block_section_v42<'a>(
    source: &'a str,
    _site_config: &'a SiteConfigV42,
    _parent: SectionParent,
    debug: bool,
) -> IResult<&'a str, SectionV42, ErrorTree<&'a str>> {
    // let debug = true;
    let dbg_header = "PARAGRAPH_BLOCK_SECTION";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, spans) = many1(alt((
        span_of_plain_text_for_block_paragraph,
        span_of_escaped_character,
        named_span,
        code_shorthand_span,
        emphasis_shorthand_span,
        footnote_shorthand_span,
        html_shorthand_span,
        image_shorthand_span,
        link_shorthand_span,
        mark_shorthand_span,
        strikethrough_shorthand_span,
        strong_shorthand_span,
        underline_shorthand_span,
    )))
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        SectionV42::Block(BlockSectionV42 {
            children: vec![],
            kind: "paragraph".to_string(),
            spans,
            template_list: make_template_list("paragraph", "paragraph", "default"),
        }),
    ))
}
