mod site_page_main_body {
    use minijinja::Value;
    use neopoligen::child::Child;
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::section::Section;
    use neopoligen::section_category::SectionCategory;
    use neopoligen::site::Site;
    use neopoligen::span::Span;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    #[test]
    pub fn basic_main_body_tests() {
        let file_set = FileSet::set2();
        let config = Config::set2();
        let site = Site::new(&file_set, &config);
        let response: Vec<Child> = vec![Child::Section(Section {
            key_value_attributes: BTreeMap::new(),
            flag_attributes: BTreeSet::new(),
            bounds: "full".to_string(),
            category: SectionCategory::StandardSectionFull {
                containers: vec![Child::Block(vec![
                    Span::Word {
                        text: "Test".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                        template: "spans/space.jinja".to_string(),
                    },
                    Span::Word {
                        text: "main_body".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                        template: "spans/space.jinja".to_string(),
                    },
                    Span::Word {
                        text: "output".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                ])],
            },
            template: "default".to_string(),
            r#type: "p".to_string(),
            source: "-- p\n\nTest main_body output".to_string(),
        })];

        let left = Value::from_serializable::<Vec<Child>>(&response);
        let right = site.page_main_body(&[Value::from("basic-main-body-test")]);
        assert_eq!(left, right);
    }
}
