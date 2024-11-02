use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn get_value_from_raw_pagedata_parsing<'a>(
    source: &'a str,
    target: &'a str,
) -> IResult<&'a str, Vec<&'a str>> {
    let (source, _) =
        many1(|src| move_parser_to_last_instance_of_section(src, "pagedata")).parse(source)?;
    let (source, things) = many1(|src| parse_pagedata_attr(src, target)).parse(source)?;
    Ok((source, things))
}

pub fn move_parser_to_last_instance_of_section<'a>(
    source: &'a str,
    target: &'a str,
) -> IResult<&'a str, &'a str> {
    let (source, _) = many0(tuple((
        opt(is_not("-")),
        not(tuple((tag("--"), space1, tag(target)))),
        is_a("-"),
    )))
    .parse(source)?;
    let (source, _) = tuple((opt(is_not("-")), tag("--"), space1, tag(target))).parse(source)?;
    Ok((source, ""))
}

pub fn parse_pagedata_attr<'a>(source: &'a str, target: &'a str) -> IResult<&'a str, &'a str> {
    let (source, _) = many0(tuple((
        opt(is_not("-")),
        not(tuple((tag("--"), space1, tag(target), tag(":")))),
        is_a("-"),
    )))
    .parse(source)?;
    let (source, id) = preceded(
        tuple((
            is_not("-"),
            tag("--"),
            space1,
            tag(target),
            tag(":"),
            space1,
        )),
        is_not("\n\r"),
    )
    .parse(source)?;
    Ok((source, id.trim()))
}
