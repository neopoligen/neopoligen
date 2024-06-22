use crate::span::Span;

use super::named_span;
use super::span_for_body_text;
use super::wordpart;

impl Span {
    pub fn mock1_basic_wordpard() -> Span {
        wordpart("alfa").unwrap().1
    }

    pub fn mock2_named_link_with_flag_and_attrs() -> Span {
        named_span(r#"<<link|alfa|https://www.example.com/|class: green blue|class: red|id: bravo|data-ping: bra"vo|rel: nofollow|autofocus|custom-key: custom"value|custom"flag|aria-valuenow: del"ta>>"#)
            .unwrap()
            .1
    }

    pub fn mock3_named_image() -> Span {
        named_span(r#"<<image|alfa1234|alt: This is "some quoted" alt text>>"#)
            .unwrap()
            .1
    }

    pub fn mock4_class_test() -> Span {
        named_span(r#"<<em|sample|class: alfa bravo|class: cha"rlie>>"#)
            .unwrap()
            .1
    }

    pub fn mock5_flag_with_quote_in_it() -> Span {
        named_span(r#"<<em|sample|fox"trot>>"#).unwrap().1
    }

    pub fn mock6_id_with_qutoe_in_t() -> Span {
        named_span(r#"<<em|sample|id: fox"trot>>"#).unwrap().1
    }

    pub fn mock7_nested_spans() -> Span {
        span_for_body_text(r#"<<em|alfa <<strong|bravo>> charlie>>"#)
            .unwrap()
            .1
    }
}
