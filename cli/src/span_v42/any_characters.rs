use super::PlainTextSpan;
use super::SpanV42;
use nom::bytes::complete::is_a;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;


pub fn any_tildies(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("~").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_exclamations(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("!").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_ats(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("@").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_pound_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("#").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_dollar_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("$").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_percent_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("%").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_carets(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("^").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_ampersands(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("&").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_asterisks(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("*").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_open_parens(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("(").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_close_parens(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(")").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_underscores(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("_").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_hyphens(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("-").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_plus_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("+").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_equal_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("=").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_open_braces(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("{").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_close_braces(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("}").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_open_brackets(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("[").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_close_brackets(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("]").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_pipes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_back_slashes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("\\").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_colons(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(":").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_semicolons(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(";").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_double_quotes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("\"").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_single_quotes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("'").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_commas(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(",").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_less_thans(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("<").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_greater_thans(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(">").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_periods(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(".").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_forward_slashes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("/").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_question_marks(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("?").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}
