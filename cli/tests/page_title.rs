mod page_title {
    use neopoligen_cli::site::Site;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn page_title_basic() {
        let site = Site::site1();
        let left = Some("Site 1 Home Page".to_string());
        let right = site.page_title("s1_index");
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_title_for_missing_page() {
        let site = Site::site1();
        let left = Some("(missing page)".to_string());
        let right = site.page_title("page_id_that_does_not_exist");
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_title_from_content() {
        let site = Site::site1();
        let left = Some("This Is A Title From A Bookmark Attribute".to_string());
        let right = site.page_title("s1_title_from_content");
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_title_from_first_few_words() {
        let site = Site::site1();
        let left = Some("This is a title from the".to_string());
        let right = site.page_title("s1_title_from_text");
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_title_from_id() {
        let site = Site::site1();
        let left = Some("only_metadata".to_string());
        let right = site.page_title("s1_only_metadata");
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_title_in_metadata() {
        let site = Site::site1();
        let left = Some("This is the override title from metadata".to_string());
        let right = site.page_title("s1_title_in_metadata");
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_title_with_inline_span() {
        let site = Site::site1();
        let left = Some("Title With Inline Span".to_string());
        let right = site.page_title("s1_title_with_inline_span");
        assert_eq!(left, right);
    }

    #[test]
    pub fn page_title_with_nested_spans() {
        let site = Site::site1();
        let left = Some("Nested Span Test".to_string());
        let right = site.page_title("s1_title_with_nested_spans");
        assert_eq!(left, right);
    }
}
