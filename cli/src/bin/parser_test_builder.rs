use minijinja::syntax::SyntaxConfig;
use minijinja::{context, Environment, Value};
use std::env::current_exe;
use std::fs;

fn main() {
    let token_set = vec![
        ("``", "``"),
        // ("<<", ">>"), TODO: set up named spans so they work with no content
        ("^^", "^^"),
    ];
    let shorthand_base_cases = make_shorthand_base_cases(token_set);
    output_file(shorthand_base_cases);
}

fn output_file(shorthand_base_cases: Vec<String>) {
    if let Ok(path) = current_exe() {
        let output_path = path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("tests")
            .join("parser_tests")
            .join("mod.rs");
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        env.add_template_owned("test_template", tmpl()).unwrap();
        let skeleton = env.get_template("test_template").unwrap();
        let output = skeleton
            .render(context!(
                shorthand_base_cases => Value::from_serialize(shorthand_base_cases)
            ))
            .unwrap();
        let _ = fs::write(output_path, output);
    }
}

fn make_shorthand_base_cases(token_set: Vec<(&str, &str)>) -> Vec<String> {
    token_set
        .iter()
        .map(|(start, end)| shorthand_base_cases(start, end))
        .flatten()
        .enumerate()
        .map(|(index, value)| format!(r#"#[case({}, "{}")]"#, index + 1, value))
        .collect::<Vec<String>>()
}

fn shorthand_base_cases(start_token: &str, end_token: &str) -> Vec<String> {
    let source = r#"alfa
alfa bravo
alfa-bravo
alfa_bravo
alfa`bravo
alfa:bravo


    #[case("``alfa\\`bravo``", 0, "escaped backtick in text")]
    #[case("``alfa\\|bravo``", 0, "escaped pipe in text")]
    #[case("``alfa\\\\bravo``", 0, "escaped backslash in text")]
    #[case("``alfa:bravo``", 0, "colon in text")]
    #[case("``alfa: bravo``", 0, "colon in text before space")]
    #[case("``alfa :bravo``", 0, "colon in text after space")]
    #[case("``alfa\\|bravo``", 0, "escaped pipe in text")]
    #[case("``alfa\\`bravo``", 0, "escaped backtick in text")]
    #[case("``alfa|bravo``", 1, "single flag attr")]
    #[case("``alfa|bravo charlie``", 1, "space in flag")]
    #[case("``alfa|bravo`charlie``", 1, "single backtick in flag")]
    #[case("``alfa|bravo\ncharlie``", 1, "newline in flag")]
    #[case("``alfa|bravo\\charlie``", 1, "non-escaped baskslash in flag")]
    #[case("``alfa|bravo\\|charlie``", 1, "escaped pipe in flag")]
    #[case("``alfa|bravo\\`charlie``", 1, "escaped backtick in flag")]

"#;
    source
        .lines()
        .map(|line| format!("{}{}{}", start_token, line, end_token))
        .collect()
}

fn tmpl() -> String {
    r#"use neopoligengine::span::*;
use rstest::rstest;

#[rstest]
[! for case in shorthand_base_cases !][@ case @]
[! endfor !]
fn generated_shorthand_base_cases(#[case] number: usize, #[case] source: &str) {
    assert!(base_span_for_all_text(source).is_ok());
}
    "#
    .to_string()
}
