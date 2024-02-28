// NOTE: This is mainly a troubleshooting test
// to help zero in on where errors are occurring.
// Once a specific error location has been
// identified a new test should be written
// for the function that has the issue

mod parsing_basic_tests {
    // use minijinja::Value;
    // use neopoligen::child::Child;
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::builder::Builder;
    // use neopoligen::section::Section;
    // use neopoligen::section_category::SectionCategory;
    use neopoligen::site::Site;
    // use neopoligen::span::Span;
    // use pretty_assertions::assert_eq;
    // use std::collections::BTreeMap;
    // use std::collections::BTreeSet;

    #[test]
    pub fn basic() {
        let file_set = FileSet::parsing_tests();
        let config = Config::parsing_tests();
        let site = Site::new(&file_set, &config);
        let builder = Builder::new(file_set, &config);

        
        // let ast = &site.pages.get("home-page").unwrap().ast;
        // dbg!(ast);

        // let left = Value::from_serializable::<Vec<Child>>(&response);
        // let right = site.pages.get("home-page");
        // assert_eq!(left, right);

    }
}
