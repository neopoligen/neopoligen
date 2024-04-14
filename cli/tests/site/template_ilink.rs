use minijinja::Value;
use neopoligengine::config::Config;
use neopoligengine::file_set::FileSet;
use neopoligengine::site::Site;
use pretty_assertions::assert_eq;

#[test]
pub fn template_ilink_return_text_for_same_page() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some(format!(r#"Lorem Ipsum"#));
    let right = site.ilink(&[
        Value::from("ttss0010"),
        Value::from("ttss0010"),
        Value::from("Lorem Ipsum"),
    ]);
    assert_eq!(left, right);
}

#[test]
pub fn template_ilink_to_another_page() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = Some(format!(
        r#"<a href="{}">{}</a>"#,
        "/en/ttss0020/?title-from-title-section", "Lorem Ipsum"
    ));
    let right = site.ilink(&[
        Value::from("ttss0010"),
        Value::from("ttss0020"),
        Value::from("Lorem Ipsum"),
    ]);
    assert_eq!(left, right);
}
