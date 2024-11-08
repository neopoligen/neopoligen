use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn plain_text_span_for_block_paragraph(
    source: &str,
) -> IResult<&str, Vec<&str>, ErrorTree<&str>> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_single_newline_as_space,
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

// pub fn plain_text_string_with_line_spaces_and_single_newlines(
//     source: &str,
// ) -> IResult<&str, Vec<&str>, ErrorTree<&str>> {
//     let (source, results) = many1(alt((
//         plain_text_string_base,
//         space1,
//         plain_text_single_newline_as_space,
//     )))
//     .context("")
//     .parse(source)?;
//     Ok((source, results))
// }

pub fn plain_text_string_base(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = alt((
        is_not(" \t\r\n~`*^![]{}<>_#|:\\"),
        terminated(tag("`"), not(tag("`"))),
        terminated(tag("~"), not(tag("~"))),
        terminated(tag("!"), not(tag("!"))),
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

// pub fn plain_text_string_with_line_spaces(
//     source: &str,
// ) -> IResult<&str, Vec<&str>, ErrorTree<&str>> {
//     let (source, results) = many1(alt((
//         plain_text_string_base,
//         plain_text_space1_as_single_space,
//     )))
//     .context("")
//     .parse(source)?;
//     Ok((source, results))
// }

pub fn plain_text_single_newline_as_space(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, _) = pair(tag("\n"), not(tag("\n"))).context("").parse(source)?;
    Ok((source, " "))
}

pub fn plain_text_space1_as_single_space(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, _) = space1.context("").parse(source)?;
    Ok((source, " "))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa", &["alfa"], "")]
    #[case("alfa bravo", &["alfa", " ", "bravo"], "")]
    #[case("alfa    bravo", &["alfa", " ", "bravo"], "")]
    #[case("alfa\nbravo", &["alfa", " ", "bravo"], "")]
    #[case("alfa\n\nbravo", &["alfa"], "\n\nbravo")]
    #[case("alfa~bravo", &["alfa", "~", "bravo"], "")]
    #[case("alfa|bravo", &["alfa", "|", "bravo"], "")]
    #[case("alfa||bravo", &["alfa", "||", "bravo"], "")]
    #[case("alfa:bravo", &["alfa", ":", "bravo"], "")]
    #[case("alfa::bravo", &["alfa", "::", "bravo"], "")]
    #[case("alfa~~bravo~~", &["alfa"], "~~bravo~~")]
    #[case("alfa ~~bravo~~", &["alfa", " "], "~~bravo~~")]
    fn solo_plain_text_span_for_block(
        #[case] source: &str,
        #[case] got: &[&str],
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got.to_vec());
        let parsed = plain_text_span_for_block_paragraph(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    #[rstest]
    #[case("|", "|", "")]
    #[case("||", "||", "")]
    fn plain_text_any_pipes_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = plain_text_any_pipes(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    #[rstest]
    #[case("|", "|", "")]
    fn plain_text_single_pipe_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = plain_text_single_pipe(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    // #[rstest]
    // #[case("alfa", &["alfa"], "")]
    // #[case("alfa bravo", &["alfa", " ", "bravo"], "")]
    // #[case("alfa bravo\ncharlie", &["alfa", " ", "bravo", " ", "charlie"], "")]
    // #[case("alfa bravo\n\ncharlie", &["alfa", " ", "bravo"], "\n\ncharlie")]
    // #[case("alfa~bravo", &["alfa", "~", "bravo"], "")]
    // fn plain_text_string_with_line_spaces_and_single_newlines_tests(
    //     #[case] source: &str,
    //     #[case] got: &[&str],
    //     #[case] remainder: &str,
    // ) {
    //     let matcher = (remainder, got.to_vec());
    //     let parsed = plain_text_string_with_line_spaces_and_single_newlines(source).unwrap();
    //     assert_eq!(matcher, parsed);
    // }

    #[rstest]
    #[case("alfa", "alfa", "")]
    #[case("alfa bravo", "alfa", " bravo")]
    #[case("alfa\nbravo", "alfa", "\nbravo")]
    #[case("`bravo", "`", "bravo")]
    #[case("~bravo", "~", "bravo")]
    #[case("!bravo", "!", "bravo")]
    #[case("#bravo", "#", "bravo")]
    #[case("^bravo", "^", "bravo")]
    #[case("*bravo", "*", "bravo")]
    #[case("[bravo", "[", "bravo")]
    #[case("]bravo", "]", "bravo")]
    #[case("{bravo", "{", "bravo")]
    #[case("}bravo", "}", "bravo")]
    #[case("<bravo", "<", "bravo")]
    #[case(">bravo", ">", "bravo")]
    #[case("_bravo", "_", "bravo")]
    fn plain_text_string_base_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = plain_text_string_base(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    // #[rstest]
    // #[case("alfa", &["alfa"], "")]
    // #[case("alfa bravo", &["alfa", " ", "bravo"], "")]
    // #[case("alfa bravo\ncharlie", &["alfa", " ", "bravo"], "\ncharlie")]
    // fn plain_text_string_with_line_spaces_tests(
    //     #[case] source: &str,
    //     #[case] got: &[&str],
    //     #[case] remainder: &str,
    // ) {
    //     let matcher = (remainder, got.to_vec());
    //     let parsed = plain_text_string_with_line_spaces(source).unwrap();
    //     assert_eq!(matcher, parsed);
    // }

    #[rstest]
    #[case("\n", " ", "")]
    fn plain_text_single_newline_as_space_tests(
        #[case] source: &str,
        #[case] got: &str,
        #[case] remainder: &str,
    ) {
        let matcher = (remainder, got);
        let parsed = plain_text_single_newline_as_space(source).unwrap();
        assert_eq!(matcher, parsed);
    }

    #[rstest]
    #[case("\n\n")]
    fn invalid_plain_text_single_newline_as_space_tests(#[case] source: &str) {
        let parsed = plain_text_single_newline_as_space(source);
        if parsed.is_err() {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    // #[rstest]
    // #[case(" ", " ", "")]
    // fn solo_plain_text_space1_not_newline_tests(
    //     #[case] source: &str,
    //     #[case] got: &str,
    //     #[case] remainder: &str,
    // ) {
    //     let matcher = (remainder, got);
    //     let parsed = plain_text_space1_not_newline(source).unwrap();
    //     assert_eq!(matcher, parsed);
    // }

    //
}
