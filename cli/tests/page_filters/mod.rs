use neopoligengine::page_filters::*;
use pretty_assertions::assert_eq;

#[test]
fn parse_include_filter() {
    let source = "status:published";
    let left = PageFilter::Status {
        exclude: false,
        value: "published".to_string(),
    };
    let right = PageFilter::parse(source);
    assert_eq!(left, right);
}
