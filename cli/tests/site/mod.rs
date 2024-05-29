use minijinja::Value;
use neopoligengine::site_v2::SiteV2;
use pretty_assertions::assert_eq;

// #[test]
// fn sort_pages_by_date() {
//     let site = SiteV2::mock1();
//     let left = Value::from_serialize(vec![
//         "delta123".to_string(),
//         "alfa1234".to_string(),
//         "hotel123".to_string(),
//         "foxtrot1".to_string(),
//         "golf1234".to_string(),
//         "echo1234".to_string(),
//         "bravo123".to_string(),
//     ]);
//     let right = site.get_pages_by_date(&[]).unwrap();
//     assert_eq!(left, right);
// }

#[test]
fn collection_by_date_basic() {
    let site = SiteV2::mock1();
    let left = Value::from_serialize(vec![
        "delta123".to_string(),
        "alfa1234".to_string(),
        "hotel123".to_string(),
        "foxtrot1".to_string(),
        "golf1234".to_string(),
        "echo1234".to_string(),
        "bravo123".to_string(),
    ]);
}

#[test]
fn collection_by_title_with_date_basic() {
    // to _with_date so the values returned match
    // what you'd get from collection_by_date.
    // then also have collection_by_title that
    // returns everything even if it doesn't have
    // a date (there's no way to match this with
    // collection_by_date since pages without
    // a date aren't included in that
}
