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
        ("\\\\", "back_slash", "back_slashes"), 
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
        ]

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
    fn single_character() {
        let source = "$CHARalfa";
        let left = (
            "alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "$CHAR".to_string(),
            }),
        );
        let right = any_$NAME2(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple() {
        let source = "$CHAR$CHARbravo";
        let left = (
            "bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "$CHAR$CHAR".to_string(),
            }),
        );
        let right = any_$NAME2(source).unwrap();
        assert_eq!(left, right);
    }
"""

#output = template.substitute(data)
#print(output)



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

