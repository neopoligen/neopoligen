use minijinja::Value;
use neopoligengine::site_v2::SiteV2;
use pretty_assertions::assert_eq;

#[test]
fn sort_pages_by_date() {
    let site = SiteV2::mock1();
    let left = Value::from_serialize(vec![
        "delta7262".to_string(),
        "abcd1234".to_string(),
        "bravo123".to_string(),
    ]);
    let right = site.get_pages_by_date(&[]).unwrap();
    assert_eq!(left, right);
}
