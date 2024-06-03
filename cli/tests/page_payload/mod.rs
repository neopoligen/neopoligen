use neopoligengine::page_v39::*;
use pretty_assertions::assert_eq;

#[test]
fn get_id() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = "20240101alfa1234".to_string();
    let right = p.payload.unwrap().id.unwrap();
    assert_eq!(left, right);
}
