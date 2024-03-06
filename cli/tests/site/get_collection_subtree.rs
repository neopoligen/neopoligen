use minijinja::Value;
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;

#[test]
pub fn subtree_from() {
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
