use super::PlainTextSpan;
use super::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
// use nom::character::complete::multispace1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn span_of_plain_text_for_shorthand_attr_value(
    source: &str,
) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_single_line_ending_as_space,
        plain_text_any_colons,
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: results.join("").to_string(),
        }),
    ))
}

pub fn span_of_plain_text_for_named_span_flag(
    source: &str,
) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_any_colons,
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: results.join("").to_string(),
        }),
    ))
}

pub fn span_of_plain_text_for_named_span_content(
    source: &str,
) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_single_line_ending_as_space,
        plain_text_any_colons,
        escaped_character,
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: results.join("").to_string(),
        }),
    ))
}

pub fn span_of_plain_text_for_section_key_value_attr_value(
    source: &str,
) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_any_pipes,
        plain_text_any_colons,
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: results.join("").to_string(),
        }),
    ))
}

pub fn span_of_plain_text_for_block_paragraph(
    source: &str,
) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, results) = plain_text_span_for_block_paragraph
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: results.join(""),
        }),
    ))
}

pub fn span_of_escaped_character(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = escaped_character.context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(super::EscapedCharacterSpanV42 {
            kind: "escaped-character".to_string(),
            text: result.to_string(),
        }),
    ))
}

pub fn plain_text_span_for_block_paragraph(
    source: &str,
) -> IResult<&str, Vec<&str>, ErrorTree<&str>> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_single_line_ending_as_space,
        plain_text_any_pipes,
        plain_text_any_colons,
    )))
    .context("")
    .parse(source)?;
    Ok((source, results))
}

pub fn plain_text_any_pipes(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = is_a("|").context("").parse(source)?;
    Ok((source, result))
}

pub fn plain_text_single_pipe(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = terminated(tag("|"), not(tag("|")))
        .context("")
        .parse(source)?;
    Ok((source, result))
}

pub fn plain_text_any_colons(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = is_a(":").context("").parse(source)?;
    Ok((source, result))
}

pub fn plain_text_single_colon(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = terminated(tag(":"), not(tag(":")))
        .context("")
        .parse(source)?;
    Ok((source, result))
}

pub fn plain_text_string_base(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = alt((
        is_not(" \t\r\n`~!@#^*[]{}<>_|:\\"),
        terminated(tag("`"), not(tag("`"))),
        terminated(tag("~"), not(tag("~"))),
        terminated(tag("!"), not(tag("!"))),
        terminated(tag("@"), not(tag("@"))),
        terminated(tag("#"), not(tag("#"))),
        terminated(tag("^"), not(tag("^"))),
        terminated(tag("*"), not(tag("*"))),
        terminated(tag("["), not(tag("["))),
        terminated(tag("]"), not(tag("]"))),
        terminated(tag("{"), not(tag("{"))),
        terminated(tag("}"), not(tag("}"))),
        terminated(tag("<"), not(tag("<"))),
        terminated(tag(">"), not(tag(">"))),
        terminated(tag("_"), not(tag("_"))),
    ))
    .context("")
    .parse(source)?;
    Ok((source, result))
}

pub fn plain_text_single_line_ending_as_space(
    source: &str,
) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, _) = tuple((space0, tag("\n"), not(pair(space0, tag("\n")))))
        .context("")
        .parse(source)?;
    Ok((source, " "))
}

pub fn plain_text_space1_as_single_space(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, _) = space1.context("").parse(source)?;
    let (source, _) = not(alt((line_ending, eof))).context("").parse(source)?;
    Ok((source, " "))
}

pub fn escaped_character(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = alt((
        preceded(tag("\\"), tag(":")),
        preceded(tag("\\"), tag("|")),
        preceded(tag("\\"), tag("`")),
        preceded(tag("\\"), tag("~")),
        preceded(tag("\\"), tag("!")),
        preceded(tag("\\"), tag("#")),
        preceded(tag("\\"), tag("^")),
        preceded(tag("\\"), tag("*")),
        preceded(tag("\\"), tag("[")),
        preceded(tag("\\"), tag("]")),
        preceded(tag("\\"), tag("{")),
        preceded(tag("\\"), tag("}")),
        preceded(tag("\\"), tag("<")),
        preceded(tag("\\"), tag(">")),
        preceded(tag("\\"), tag("_")),
        preceded(tag("\\"), tag("\\")),
    ))
    .context("")
    .parse(source)?;
    Ok((source, result))
}

// NOTE: NOTE SURE IF THIS IS NEEDED
// pub fn plain_text_end_of_block(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
//     let (source, _) = alt((
//         eof,
//         terminated(space1, eof),
//         terminated(space1, not(pair(line_ending, line_ending))),
//     ))
//     .context("")
//     .parse(source)?;
//     Ok((source, ""))
// }
