use neopoligengine::helpers::*;
use pretty_assertions::assert_eq;

#[test]
fn clean_for_url_basic() {
    let source = "  Some 1234 --  $ x ' y Path ";
    let left = "some-1234-x-y-path";
    let right = clean_for_url(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn clean_for_url_comp_multiple_dashes() {
    let source = "alfa--bravo--charlie";
    let left = "alfa-bravo-charlie";
    let right = clean_for_url(source).unwrap();
    assert_eq!(left, right);
}
