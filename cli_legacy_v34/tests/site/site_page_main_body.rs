// DEPRECATED: This is replaced by just using the templates
// and the AST. leaving here for now to see about making
// sure the test is covered later
//
//mod site_page_main_body {
//    use minijinja::Value;
//    use neopoligengine::child::Child;
//    use neopoligengine::config::Config;
//    use neopoligengine::file_set::FileSet;
//    use neopoligengine::section::Section;
//    use neopoligengine::section_category::SectionCategory;
//    use neopoligengine::site::Site;
//    use neopoligengine::span::Span;
//    use pretty_assertions::assert_eq;
//    use std::collections::BTreeMap;
//    use std::collections::BTreeSet;
//    #[test]
//    #[ignore]
//    pub fn basic_main_body_tests() {
//        let file_set = FileSet::set2();
//        let config = Config::set2();
//        let site = Site::new(&file_set, &config);
//        let response: Vec<Child> = vec![Child::Section(Section {
//            key_value_attributes: BTreeMap::new(),
//            flag_attributes: BTreeSet::new(),
//            bounds: "full".to_string(),
//            category: SectionCategory::StandardSectionFull {
//                containers: vec![Child::Block(vec![
//                    Span::WordSegment {
//                        text: "Test".to_string(),
//                        template: "spans/word_segment.neojinja".to_string(),
//                    },
//                    Span::Space {
//                        text: " ".to_string(),
//                        template: "spans/space.neojinja".to_string(),
//                    },
//                    Span::WordSegment {
//                        text: "main_body".to_string(),
//                        template: "spans/word_segment.neojinja".to_string(),
//                    },
//                    Span::Space {
//                        text: " ".to_string(),
//                        template: "spans/space.neojinja".to_string(),
//                    },
//                    Span::WordSegment {
//                        text: "output".to_string(),
//                        template: "spans/word_segment.neojinja".to_string(),
//                    },
//                ])],
//            },
//            template: "default".to_string(),
//            r#type: "p".to_string(),
//            source: "-- p\n\nTest main_body output".to_string(),
//        })];
//        let left = Value::from_serialize::<Vec<Child>>(response);
//        let right = site.page_main_body(&[Value::from("basic-main-body-test")]);
//        assert_eq!(left, right);
//    }
//}
