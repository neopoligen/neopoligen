mod page_title {
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn page_title_basic() {
        let site = Site::site1();
        let left = "Site 1 Home Page";
        let right = site.page_title("site1_index");
        assert_eq!(left, right);
    }
}
