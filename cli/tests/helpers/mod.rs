use neopoligengine::helpers::*;
use pretty_assertions::assert_eq;

#[test]
fn clean_for_url_basic() {
    let source = "  Some 1234 --  $ x ' y Path ";
    let left = "some-1234-x-y-path";
    let right = clean_for_url(source).unwrap();
    assert_eq!(left, right);
}
