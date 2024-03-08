use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;
use std::collections::BTreeSet;

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
    let right = site.pages.get("ttss0030").unwrap().title.clone();
    assert_eq!(left, right);
}

#[test]
fn title_from_any_section() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("Title From Any Section".to_string());
    let right = site.pages.get("ttss0040").unwrap().title.clone();
    assert_eq!(left, right);
}

#[test]
fn title_from_first_few_words_with_elipses() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("Title from the first few words of a section lorem ipsum verde...".to_string());
    let right = site.pages.get("ttss0050").unwrap().title.clone();
    assert_eq!(left, right);
}

#[test]
fn title_from_first_few_words_that_does_not_get_truncated() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("First words short title".to_string());
    let right = site.pages.get("ttss0051").unwrap().title.clone();
    assert_eq!(left, right);
}

#[test]
fn title_from_first_few_words_do_not_add_etra_dot_if_break_is_at_a_period() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left =
        Some("First few words. There should only be three dots here not four...".to_string());
    let right = site.pages.get("ttss0052").unwrap().title.clone();
    assert_eq!(left, right);
}

#[test]
fn title_from_id_as_fallback() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some("ttss0060".to_string());
    let right = site.pages.get("ttss0060").unwrap().title.clone();
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

#[test]
fn html_link() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some(
        r#"<a href="/en/ttss0020/?title-from-title-section">Title From Title Section</a>"#
            .to_string(),
    );
    let right = site.pages.get("ttss0020").unwrap().html_link.clone();
    assert_eq!(left, right);
}

#[test]
fn path_parts() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = vec!["title-from-title-section.neo".to_string()];
    let right = site.pages.get("ttss0020").unwrap().path_parts.clone();
    assert_eq!(left, right);
}

#[test]
fn load_default_type() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = &Some("post".to_string());
    let right = &site.pages.get("ttss0100").unwrap().r#type;
    assert_eq!(left, right);
}

#[test]
fn load_custom_type_from_metadata() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = &Some("custom-page-type".to_string());
    let right = &site.pages.get("ttss0090").unwrap().r#type;
    assert_eq!(left, right);
}

#[test]
fn load_type_from_first_folder() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = &Some("type-from-first-folder".to_string());
    let right = &site.pages.get("ttss0110").unwrap().r#type;
    assert_eq!(left, right);
}

#[test]
fn load_default_status() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = &Some("published".to_string());
    let right = &site.pages.get("ttss0070").unwrap().status;
    assert_eq!(left, right);
}

#[test]
fn load_custom_status() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = &Some("custom-page-status".to_string());
    let right = &site.pages.get("ttss0090").unwrap().status;
    assert_eq!(left, right);
}

#[test]
fn load_tags() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let mut left = BTreeSet::new();
    left.insert("ttss0070".to_string());
    left.insert("tag-from-folder".to_string());
    left.insert("tag-from-tags-section".to_string());
    left.insert("published".to_string());
    let right = site.pages.get("ttss0070").unwrap().tags.clone();
    assert_eq!(left, right);
}

#[test]
fn stylesheets() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left: &Vec<String> = &vec![r#"body { color: goldenrod; }"#.to_string()];
    let right = &site.pages.get("ttss0120").unwrap().stylesheets;
    assert_eq!(left, right);
}

#[test]
fn scripts() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left: &Vec<String> = &vec![r#"console.log("ping")"#.to_string()];
    let right = &site.pages.get("ttss0130").unwrap().scripts;
    assert_eq!(left, right);
}

#[test]
fn head() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left: &Vec<String> = &vec![r#"<!-- content for head -->"#.to_string()];
    let right = &site.pages.get("ttss0140").unwrap().head;
    assert_eq!(left, right);
}
