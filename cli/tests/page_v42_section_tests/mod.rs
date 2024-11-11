use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use minijinja::Value;
use neopoligengine::page_v42::PageV42b;
use pretty_assertions::assert_eq;
use serde_json::Value as SerdeValue;
use std::collections::BTreeMap;

const SOURCE_DIR: Dir = include_dir!("tests/page_v42_section_tests/valid/input");

lazy_static! {
    static ref SOURCE_FILES: BTreeMap<String, String> = {
        let glob = "**/*.neo";
        let mut btm = BTreeMap::new();
        for entry in SOURCE_DIR.find(glob).unwrap() {
            let data = SOURCE_DIR.get_file(entry.path());
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

const OUTPUT_DIR: Dir = include_dir!("tests/page_v42_section_tests/valid/output");

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
fn solo_page_section_integration_tests() {
    for (id, _source) in SOURCE_FILES.iter() {
        match OUTPUT_FILES.get(id) {
            Some(output) => {
                match serde_json5::from_str::<SerdeValue>(output) {
                    Ok(json5) => {
                        let page = PageV42b::mock_from_section_tests(id);
                        match page.parsing_issue {
                            Some(issue) => {
                                dbg!(issue);
                                ()
                            }
                            None => (),
                        };
                        let check_value = Value::from_serialize(json5);
                        assert_eq!(
                            page.sections.unwrap(),
                            check_value,
                            "##### ERROR ##### - {}",
                            &id
                        );
                    }
                    Err(e) => {
                        dbg!(e);
                        assert!(false)
                    }
                };
            }
            None => {
                assert!(false);
            }
        }
    }
}
