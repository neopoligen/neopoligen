use neopoligengine::page_filters::*;
use pretty_assertions::assert_eq;

#[test]
fn parse_include_status() {
    let source = "status:published";
    let left = Some(PageFilter::Status {
        exclude: false,
        value: "published".to_string(),
    });
    let right = PageFilter::parse(source);
    assert_eq!(left, right);
}

#[test]
fn parse_exclude_status() {
    let source = "status:!published";
    let left = Some(PageFilter::Status {
        exclude: true,
        value: "published".to_string(),
    });
    let right = PageFilter::parse(source);
    assert_eq!(left, right);
}
