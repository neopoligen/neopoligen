mod site_page_place_section {
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
    pub fn basic_place_section() {
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
                        text: "Basic".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                        template: "spans/space.jinja".to_string(),
                    },
                    Span::Word {
                        text: "Place".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                        template: "spans/space.jinja".to_string(),
                    },
                    Span::Word {
                        text: "Section".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                        template: "spans/space.jinja".to_string(),
                    },
                    Span::Word {
                        text: "Test".to_string(),
                        template: "spans/word.jinja".to_string(),
                    },
                ])],
            },
            template: "default".to_string(),
            r#type: "title".to_string(),
            source: "-- title\n\nBasic Place Section Test".to_string(),
        })];

        let left = Value::from_serializable::<Vec<Child>>(&response);
        let right = site.page_place_section(&[
            Value::from("basic-place-section-test"), 
            Value::from("title")
        ]);
        assert_eq!(left, right);
    }
}
