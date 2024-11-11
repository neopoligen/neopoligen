use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use minijinja::Value;
use neopoligengine::section_v42::checklist::*;
use neopoligengine::section_v42::*;
use neopoligengine::site_config::*;
use pretty_assertions::assert_eq;
use serde_json::Value as SerdeValue;
use serde_json5;
use std::collections::BTreeMap;

const INPUT_DIR: Dir = include_dir!("tests/section_v42_tests/checklist/valid/input");
const OUTPUT_DIR: Dir = include_dir!("tests/section_v42_tests/checklist/valid/output");

lazy_static! {
    static ref INPUT_FILES: BTreeMap<String, String> = {
        let glob = "**/*.neo";
        let mut btm = BTreeMap::new();
        for entry in INPUT_DIR.find(glob).unwrap() {
            let data = INPUT_DIR.get_file(entry.path());
            btm.insert(
                entry
                    .path()
                    .display()
                    .to_string()
                    .trim_end_matches(".neo")
                    .to_string(),
                data.expect(
                    format!(
                        "Could not open built-in include: {}",
                        entry.path().display().to_string()
                    )
                    .as_str(),
                )
                .contents_utf8()
                .unwrap()
                .to_string(),
            );
        }
        btm
    };
}

lazy_static! {
    static ref OUTPUT_FILES: BTreeMap<String, String> = {
        let glob = "**/*.json5";
        let mut btm = BTreeMap::new();
        for entry in OUTPUT_DIR.find(glob).unwrap() {
            let data = OUTPUT_DIR.get_file(entry.path());
            btm.insert(
                entry
                    .path()
                    .display()
                    .to_string()
                    .trim_end_matches(".json5")
                    .to_string(),
                data.expect(
                    format!(
                        "Could not open built-in include: {}",
                        entry.path().display().to_string()
                    )
                    .as_str(),
                )
                .contents_utf8()
                .unwrap()
                .to_string(),
            );
        }
        btm
    };
}

#[test]
fn solo_checklist_section_tests() {
    for (id, source) in INPUT_FILES.iter() {
        match OUTPUT_FILES.get(id) {
            Some(output) => {
                match serde_json5::from_str::<SerdeValue>(output) {
                    Ok(json5) => {
                        let site_config = SiteConfigV42::mock1_basic();
                        let parent = SectionParent::Page;
                        let debug = false;
                        let left = Value::from_serialize(json5);
                        let right = Value::from_serialize(
                            checklist_section_v42(source.trim(), &site_config, parent, debug)
                                .unwrap()
                                .1,
                        );
                        assert_eq!(left, right, "Error in: {}", id);
                    }
                    Err(e) => {
                        dbg!(e);
                        assert!(false, "Error in: {}", id);
                    }
                };
            }
            None => {
                assert!(false, "Error in: {}", id);
            }
        }
    }
}
