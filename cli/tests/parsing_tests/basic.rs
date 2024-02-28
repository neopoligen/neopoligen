// NOTE: This is mainly a troubleshooting test
// to help zero in on where errors are occurring.
// Once a specific error location has been
// identified a new test should be written
// for the function that has the issue

mod parsing_basic_tests {
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::builder::Builder;
    use neopoligen::site::Site;

    #[test]
    pub fn basic() {
        let file_set = FileSet::parsing_tests();
        let config = Config::parsing_tests();
        let _site = Site::new(&file_set, &config);
        let _builder = Builder::new(file_set, &config);
    }
}
