use minijinja::Value;
use neopoligengine::site_v2::SiteV2;
use pretty_assertions::assert_eq;

#[test]
fn build_time_string() {
    let site = SiteV2::mock1();
    let left = Value::from("2015-05-15T00:00:00Z".to_string());
    let right = site.build_time().unwrap();
    assert_eq!(left, right);
}

#[test]
fn collection_by_date_status_published() {
    let site = SiteV2::mock1();
    let left = Value::from_serialize(vec![
        "delta123".to_string(),
        "alfa1234".to_string(),
        "hotel123".to_string(),
        "golf1234".to_string(),
        "foxtrot1".to_string(),
    ]);
    let args = [Value::from_serialize(vec!["status:published"])];
    let right = site.collection_by_date(&args).unwrap();
    assert_eq!(left, right);
}

#[test]
fn collection_by_date_status_not_published() {
    let site = SiteV2::mock1();
    let left = Value::from_serialize(vec!["echo1234".to_string(), "charlie1".to_string()]);
    let args = [Value::from_serialize(vec!["status:!published"])];
    let right = site.collection_by_date(&args).unwrap();
    assert_eq!(left, right);
}

#[test]
fn collection_by_date_type_post() {
    let site = SiteV2::mock1();
    let left = Value::from_serialize(vec![
        "alfa1234".to_string(),
        "hotel123".to_string(),
        "golf1234".to_string(),
        "foxtrot1".to_string(),
        "charlie1".to_string(),
    ]);
    let args = [Value::from_serialize(vec!["type:post"])];
    let right = site.collection_by_date(&args).unwrap();
    assert_eq!(left, right);
}

#[test]
fn collection_by_date_type_not_post() {
    let site = SiteV2::mock1();
    let left = Value::from_serialize(vec!["delta123".to_string(), "echo1234".to_string()]);
    let args = [Value::from_serialize(vec!["type:!post"])];
    let right = site.collection_by_date(&args).unwrap();
    assert_eq!(left, right);
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

#[test]
fn page_date_basic() {
    let site = SiteV2::mock1();
    let left = Value::from("2024-05-20T10:11:12-04:00".to_string());
    let right = site.page_date_for_feed(&[Value::from("alfa1234")]).unwrap();
    assert_eq!(left, right);
}

#[test]
fn site_uuid_basic() {
    let site = SiteV2::mock1();
    let left = Value::from("f2379517-fef2-587f-a259-81aeb4a9b7fd".to_string());
    let right = site.uuid().unwrap();
    assert_eq!(left, right);
}
