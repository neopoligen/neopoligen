#!/usr/bin/env python3

from string import Template

characters = [
        ("~", "tilde", "tildies"),
        ("!", "exclamation", "exclamations"),
        ("@", "at", "ats"),
        ("#", "pound_sign", "pound_signs"),
        ("$", "dollar_sign", "dollar_signs"), 
        ("%", "percent_sign", "percent_signs"), 
        ("^", "caret", "carets"), 
        ("&", "ampersand", "ampersands"), 
        ("*", "asterisk", "asterisks"), 
        ("(", "open_paren", "open_parens"), 
        (")", "close_paren", "close_parens"), 
        ("_", "underscore", "underscores"), 
        ("-", "hyphen", "hyphens"), 
        ("+", "plus_sign", "plus_signs"), 
        ("=", "equal_sign", "equal_signs"), 
        ("{", "open_brace", "open_braces"), 
        ("}", "close_brace", "close_braces"), 
        ("[", "open_bracket", "open_brackets"), 
        ("]", "close_bracket", "close_brackets"), 
        ("|", "pipe", "pipes"), 
        ("\\\\", "backslash", "backslashes"), 
        (":", "colon", "colons"), 
        (";", "semicolon", "semicolons"), 
        ('\\"', "double_quote", "double_quotes"), 
        ("'", "single_quote", "single_quotes"), 
        (",", "comma", "commas"), 
        ("<", "less_than", "less_thans"), 
        (">", "greater_than", "greater_thans"), 
        (".", "period", "periods"), 
        ("/", "forward_slash", "forward_slashes"), 
        ("?", "question_mark", "question_marks"), 
        ("`", "backtick", "backticks"), 
        ]

###################################################################

any_functions_skeleton = """
pub fn any_$NAME2(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("$CHAR").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}
"""

any_tests_skeleton = """
    #[test]
    fn single_character_$NAME1() {
        let source = "$CHAR alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "$CHAR".to_string(),
            }),
        );
        let right = any_$NAME2(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_$NAME1() {
        let source = "$CHAR$CHAR bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "$CHAR$CHAR".to_string(),
            }),
        );
        let right = any_$NAME2(source).unwrap();
        assert_eq!(left, right);
    }
"""


with open("any_characters.rs", "w") as f:
    f.write("""use super::PlainTextSpan;
use super::SpanV42;
use nom::bytes::complete::is_a;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

""")

    for character in characters:
        data = { "CHAR": character[0], "NAME1": character[1], "NAME2": character[2] }
        template = Template(any_functions_skeleton)
        output = template.substitute(data)
        f.write(output)
    f.write("\n\n")


    f.write("""#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

""")

    for character in characters:
        data2 = { "CHAR": character[0], "NAME1": character[1], "NAME2": character[2] }
        template2 = Template(any_tests_skeleton)
        output2 = template2.substitute(data2)
        f.write(output2)
    f.write("\n\n")

    f.write("""}""")


###################################################################

escaped_functions_skeleton = '''
pub fn escaped_$NAME1(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\\\$CHAR").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "$CHAR".to_string(),
        }),
    ))
}
'''



escaped_tests_skeleton = '''
    #[test]
    fn check_escaped_$NAME1() {
        let source = "\\\\$CHAR alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: "$CHAR".to_string(),
            }),
        );
        let right = escaped_$NAME1(source).unwrap();
        assert_eq!(left, right);
    }
'''


with open("escaped_characters.rs", "w") as f:
    f.write("""use super::SpanV42;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EscapedCharacterSpanV42 {
    pub text: String,
}

""")

    for character in characters:
        data = { "CHAR": character[0], "NAME1": character[1], "NAME2": character[2] }
        template = Template(escaped_functions_skeleton)
        output = template.substitute(data)
        f.write(output)
    f.write("\n\n")


    f.write("""#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

""")

    for character in characters:
        data2 = { "CHAR": character[0], "NAME1": character[1], "NAME2": character[2] }
        template2 = Template(escaped_tests_skeleton)
        output2 = template2.substitute(data2)
        f.write(output2)
    f.write("\n\n")

    f.write("""}""")



###################################################################

single_functions_skeleton = '''
pub fn single_$NAME1(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("$CHAR"), not(tag("$CHAR")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}
'''


single_tests_skeleton = '''
    #[test]
    fn single_at_eof_$NAME1() {
        let source = "$CHAR";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "$CHAR".to_string(),
            }),
        );
        let right = single_$NAME1(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_$NAME1() {
        let source = "$CHAR alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "$CHAR".to_string(),
            }),
        );
        let right = single_$NAME1(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_$NAME2() {
        let source = "$CHAR$CHAR";
        assert!(single_$NAME1(source).is_err());
    }

'''


with open("single_characters.rs", "w") as f:
    f.write("""use crate::span_v42::*;
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

""")

    for character in characters:
        data = { "CHAR": character[0], "NAME1": character[1], "NAME2": character[2] }
        template = Template(single_functions_skeleton)
        output = template.substitute(data)
        f.write(output)
    f.write("\n\n")


    f.write("""#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

""")

    for character in characters:
        data2 = { "CHAR": character[0], "NAME1": character[1], "NAME2": character[2] }
        template2 = Template(single_tests_skeleton)
        output2 = template2.substitute(data2)
        f.write(output2)
    f.write("\n\n")

    f.write("""}""")


