use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;

#[test]
fn title_from_title_section() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("Title From Title Section".to_string());
    let right = site.pages.get("aabb0020").unwrap().title.clone();
    assert_eq!(left, right);
}

#[test]
fn title_from_metadata() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("Title From Metadata".to_string());
    let right = site.pages.get("aabb0030").unwrap().title.clone();
    assert_eq!(left, right);
}
