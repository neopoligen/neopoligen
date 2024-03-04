use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;

#[test]
#[ignore]
fn title_from_title_section() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let left = Some("Home Page".to_string());
    let right = site.pages.get("aabb0011").unwrap().title.clone();
    assert_eq!(left, right);
}

