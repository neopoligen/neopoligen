pub mod basic;
pub mod raw;
pub mod yaml;

use neopoligengine::section_v39::*;
use pretty_assertions::assert_eq;

#[test]
fn type_check() {
    let section = SectionV39::mock1_basic_full();
    let left = "title".to_string();
    let right = section.r#type;
    assert_eq!(left, right);
}

// commented out ones below are from the function
// based approach that need to be redone

// #[test]
// fn basic_test() {
//     let section = SectionV39::mock1_basic_full();
//     let left = "title";
//     let right = section.r#type();
//     assert_eq!(left, right);
// }

// #[test]
// fn bound_test() {
//     let section = SectionV39::mock1_basic_full();
//     let left = "full";
//     let right = section.bounds().unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn get_attr_none_when_no_key() {
//     let section = SectionV39::mock1_basic_full();
//     let left = None;
//     let right = section.get_attr("invalid_key");
//     assert_eq!(left, right);
// }

// #[test]
// fn get_attr_single_key() {
//     let section = SectionV39::mock2_basic_full_attrs();
//     let left = Some("show".to_string());
//     let right = section.get_attr("template");
//     assert_eq!(left, right);
// }

// #[test]
// fn get_attr_multiple_of_same_key() {
//     let section = SectionV39::mock2_basic_full_attrs();
//     let left = Some("line 1 line 2".to_string());
//     let right = section.get_attr("alt");
//     assert_eq!(left, right);
// }

// #[test]
// fn template_default() {
//     let section = SectionV39::mock1_basic_full();
//     let left = "default".to_string();
//     let right = section.template().unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn template_override() {
//     let section = SectionV39::mock2_basic_full_attrs();
//     let left = "show".to_string();
//     let right = section.template().unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn template_list_default() {
//     let section = SectionV39::mock1_basic_full();
//     let left = vec![
//         "sections/title/full/default.neoj".to_string(),
//         "sections/generic/full/default.neoj".to_string(),
//     ];
//     let right = section.template_list();
//     assert_eq!(left, right);
// }

// #[test]
// fn template_list_with_override() {
//     let section = SectionV39::mock2_basic_full_attrs();
//     let left = vec![
//         "sections/title/full/show.neoj".to_string(),
//         "sections/title/full/default.neoj".to_string(),
//         "sections/generic/full/default.neoj".to_string(),
//     ];
//     let right = section.template_list();
//     assert_eq!(left, right);
// }
