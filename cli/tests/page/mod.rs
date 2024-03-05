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
    let right = site.pages.get("ttss0020").unwrap().title.clone();
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

#[test]
fn title_from_any_section() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("Title From Any Section".to_string());
    let right = site.pages.get("aabb0040").unwrap().title.clone();
    assert_eq!(left, right);
}

#[test]
fn title_from_first_few_words() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("Title from the first few words".to_string());
    let right = site.pages.get("aabb0050").unwrap().title.clone();
    assert_eq!(left, right);
}

#[test]
fn title_from_id_as_fallback() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("aabb0060".to_string());
    let right = site.pages.get("aabb0060").unwrap().title.clone();
    assert_eq!(left, right);
}

#[test]
fn href_basic() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("/en/ttss0020/?title-from-title-section".to_string());
    let right = site.pages.get("ttss0020").unwrap().href.clone();
    assert_eq!(left, right);
}
