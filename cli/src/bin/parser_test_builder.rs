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
        .map(|(index, (source, flags, kv))| {
            format!(r#"#[case({}, "{}", {}, {})]"#, index + 1, source, flags, kv)
        })
        .collect::<Vec<String>>()
}

fn shorthand_base_cases(start_token: &str, end_token: &str) -> Vec<(String, usize, usize)> {
    // format: string, number of expected flags, number of expected key/values
    let base = vec![
        (r#"alfa bravo"#, 0, 0),
        (r#"alfa`bravo"#, 0, 0),
        (r#"alfa-bravo"#, 0, 0),
        (r#"alfa_bravo"#, 0, 0),
        (r#"alfa:bravo"#, 0, 0),
        (r#"alfa<bravo"#, 0, 0),
        (r#"alfa>bravo"#, 0, 0),
        (r#"alfa^bravo"#, 0, 0),
        (r#"alfa\\|bravo"#, 0, 0),
        (r#"alfa\\``bravo"#, 0, 0),
        (r#"alfa\\-bravo"#, 0, 0),
        (r#"alfa\\_bravo"#, 0, 0),
        (r#"alfa\\--bravo"#, 0, 0),
        (r#"alfa\\__bravo"#, 0, 0),
        (r#"alfa\\:bravo"#, 0, 0),
        (r#"alfa\\::bravo"#, 0, 0),
        (r#"alfa\\\\bravo"#, 0, 0),
    ];
    base.iter()
        .map(|(s, f, kv)| {
            (
                format!("{}{}{}", start_token, s, end_token),
                f.clone(),
                kv.clone(),
            )
        })
        .collect()

    // #[case("``alfa|bravo``", 1, "single flag attr")]
    // #[case("``alfa|bravo charlie``", 1, "space in flag")]
    // #[case("``alfa|bravo`charlie``", 1, "single backtick in flag")]
    // #[case("``alfa|bravo\ncharlie``", 1, "newline in flag")]
    // #[case("``alfa|bravo\\charlie``", 1, "non-escaped baskslash in flag")]
    // #[case("``alfa|bravo\\|charlie``", 1, "escaped pipe in flag")]
    // #[case("``alfa|bravo\\`charlie``", 1, "escaped backtick in flag")]

    // "#;
    //     source
    //         .lines()
    //         .map(|line| format!("{}{}{}", start_token, line, end_token))
    //         .collect()
}

fn tmpl() -> String {
    r#"use neopoligengine::span::*;
use neopoligengine::span_attr::*;
use rstest::rstest;

#[rstest]
[! for case in shorthand_base_cases !][@ case @]
[! endfor !]
fn generated_shorthand_base_cases(
    #[case] _number: usize, 
    #[case] source: &str, 
    #[case] flag_count: usize, 
    #[case] kv_count: usize
    ) {
    let span = shorthand(source).unwrap().1;
    let kv = span.attrs.iter().filter_map(|attr|{
        match &attr.kind {
            SpanAttrKind::KeyValue { .. } => Some(()),
            _ => None
        }
    }).collect::<Vec<()>>();
    let flags = span.attrs.iter().filter_map(|attr|{
        match &attr.kind {
            SpanAttrKind::Flag { .. } => Some(()),
            _ => None
        }
    }).collect::<Vec<()>>();
    assert_eq!(kv.len(), kv_count);
    assert_eq!(flags.len(), flag_count);
}
    "#
    .to_string()
}
