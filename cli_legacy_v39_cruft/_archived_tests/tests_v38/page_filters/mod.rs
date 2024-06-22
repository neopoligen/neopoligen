// use minijinja::Value;
// use neopoligengine::page_filters::*;
// use pretty_assertions::assert_eq;

// TODO: Move to PageFilterV2
// #[test]
// fn filter_parse_include_status() {
//     let source = "status:published";
//     let left = Some(PageFilter::Status {
//         exclude: false,
//         value: "published".to_string(),
//     });
//     let right = PageFilter::parse(source);
//     assert_eq!(left, right);
// }

// TODO: Move to PageFilterV2
// #[test]
// fn filter_parse_exclude_status() {
//     let source = "status:!published";
//     let left = Some(PageFilter::Status {
//         exclude: true,
//         value: "published".to_string(),
//     });
//     let right = PageFilter::parse(source);
//     assert_eq!(left, right);
// }

// TODO: Move to PageFilterV2
// #[test]
// fn or_set_parse_basic() {
//     let source = &[Value::from_serialize(vec!["status:test"])];
//     let left = Some(PageFilterOrSet {
//         and_groups: vec![PageFilterAndGroup {
//             filters: vec![PageFilter::Status {
//                 exclude: false,
//                 value: "test".to_string(),
//             }],
//         }],
//     });
//     let right = PageFilterOrSet::parse(source);
//     assert_eq!(left, right);
// }
