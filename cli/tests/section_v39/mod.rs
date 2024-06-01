pub mod basic;
pub mod yaml;

use minijinja::Value;
use neopoligengine::section_v39::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_section_basic_test() {
    let section = SectionV39::mock1_basic_full();
    let left = "title";
    let right = section.r#type();
    assert_eq!(left, right);
}

#[test]
fn basic_section_template() {
    let section = SectionV39::mock1_basic_full();
    let left = Value::from("default");
    let right = section.template().unwrap();
    assert_eq!(left, right);
}
