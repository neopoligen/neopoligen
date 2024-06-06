use crate::span::Span;

use super::named_span;
use super::wordpart;

impl Span {
    pub fn mock1_basic_wordpard() -> Span {
        wordpart("alfa").unwrap().1
    }

    pub fn mock2_named_link_with_flag_and_attrs() -> Span {
        named_span("<<link|alfa|https://www.example.com/|class: green|id: bravo|data-ping: bravo|rel: nofollow>>")
            .unwrap()
            .1
    }

    pub fn mock3_named_image() -> Span {
        named_span(r#"<<image|alfa1234|alt: This is "some quoted" alt text>>"#)
            .unwrap()
            .1
    }
}
