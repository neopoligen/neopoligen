use minijinja::Value;
use neopoligengine::config::Config;
use neopoligengine::file_set::FileSet;
use neopoligengine::site::Site;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
pub fn template_ilink_to_other_page() {
    let config = Config::inline();
    let mut file_set = FileSet::new();

    file_set
        .templates
        .insert("pages/post/published.jinja".to_string(), "HERE".to_string());

    file_set.pages.insert(
        PathBuf::from("leading-dir/Neopoligen/inline-test-site/content/test-file.neo"),
        r#"-- metadata
-- id: file-under-test
"#
        .to_string(),
    );

    let site = Site::new(&file_set, &config);

    // let left = Some("example".to_string());
    // let right = site.page_type(&[Value::from("page-type-in-metadata")]);
    // assert_eq!(left, right);
}
