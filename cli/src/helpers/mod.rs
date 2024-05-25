use minijinja::value::{Object, Value};
use minijinja::Error;
use regex::Regex;
use std::fmt::Display;
use std::sync::Arc;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Debug)]
pub struct Helpers {}

impl Display for Helpers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this is here")
    }
}

impl Object for Helpers {
    fn call_method(
        self: &Arc<Helpers>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "ping" => Ok(Value::from(self.ping())),
            //"highlight_code" => Ok(Value::from_serialize(&self.highlight_code(args))),
            _ => Ok(Value::from("ping")),
        }
    }
}

impl Helpers {
    pub fn ping(&self) -> String {
        "One ping only".to_string()
    }

    pub fn highlight_code(&self, args: &[Value]) -> String {
        let code = args[0].to_string();
        let lang = args[1].to_string();
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let syntax = syntax_set
            .find_syntax_by_token(&lang)
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
        let mut html_generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
        for line in LinesWithEndings::from(&self.trim_empty_lines(&code)) {
            let _ = html_generator.parse_html_for_line_which_includes_newline(line);
        }
        let initial_html = html_generator.finalize();
        let output_html: Vec<_> = initial_html
            .lines()
            .map(|line| format!(r#"<span class="line-marker"></span>{}"#, line))
            .collect();
        output_html.join("\n")
    }

    pub fn trim_empty_lines(&self, source: &str) -> String {
        let re = Regex::new(r"\S").unwrap();
        let trimmed_front = source.split("\n").fold("".to_string(), |acc, l| {
            if !acc.is_empty() {
                acc + l + "\n"
            } else {
                if re.is_match(l) {
                    l.to_string() + "\n"
                } else {
                    acc
                }
            }
        });
        trimmed_front.trim_end().to_string()
    }
}
