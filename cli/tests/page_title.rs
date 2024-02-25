mod page_title {
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn page_title_basic() {
        let site = Site::site1();
        let left = Some("Site 1 Home Page".to_string());
        let right = site.page_title("site1_index");
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_title_with_inline_span() {
        let site = Site::site1();
        let left = Some("Title With Inline Span".to_string());
        let right = site.page_title("site1_title_with_inline_span");
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_title_with_missing_page() {
        let site = Site::site1();
        let left = Some("(missing page)".to_string());
        let right = site.page_title("page_id_that_does_not_exist");
        assert_eq!(left, right);
    }
}
