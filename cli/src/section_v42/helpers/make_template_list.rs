use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use itertools::Itertools;
use std::collections::BTreeMap;

// TODO: Deprecate this
pub fn make_template_list(r#type: &str, kind: &str, template: &str) -> Vec<String> {
    let templates = vec![
        format!("sections/{}/{}.neoj", kind, template),
        format!("sections/{}/default.neoj", kind),
        format!("sections/{}/default.neoj", r#type),
    ];
    templates.into_iter().unique().collect()
}

// TODO: Add an automated test for this.
pub fn make_template_list_v42(
    r#type: &str,
    kind: &str,
    attrs: &BTreeMap<String, Vec<SectionKeyValueAttrPayloadV42>>,
) -> Vec<String> {
    let templates = match &attrs.get("template") {
        Some(attr) => {
            vec![
                format!("sections/{}/{}.neoj", kind, attr[0].text),
                format!("sections/{}/default.neoj", kind),
                format!("sections/{}/default.neoj", r#type),
            ]
        }
        None => {
            vec![
                format!("sections/{}/default.neoj", kind),
                format!("sections/{}/default.neoj", r#type),
            ]
        }
    };

    // let templates = templates
    //     .into_iter()
    //     .unique()
    //     .filter(|v| {
    //         // TODO: Make this not hard coded:
    //         let base_path = PathBuf::from(
    //             "/Users/alan/Documents/Neopoligen/alanwsmith.com/themes/neopoligen-v0.42.0",
    //         );
    //         let check_path = base_path.join(v);
    //         template_file_exists(&check_path)
    //         // dbg!(&check_path);
    //         // true
    //     })
    //     .collect::<Vec<String>>();

    // for x in &templates {
    //     println!("{}", &x);
    // }

    // dbg!(&templates);

    // let templates = vec![
    //     format!("sections/{}/default.neoj", kind),
    //     format!("sections/{}/default.neoj", r#type),
    // ];

    // dbg!(&kind);

    // let templates = vec![
    //     // format!("sections/{}/default.neoj", "p"),
    //     // format!("sections/{}/default.neoj", kind),
    //     format!("sections/{}/default.neoj", r#type),
    //     // format!("sections/basic/default.neoj"),
    //     // format!("sections/{}/default.neoj", "p"),
    // ];

    // templates

    templates.into_iter().unique().collect()
    // let templates = vec![
    //     format!("sections/{}/{}.neoj", kind, template_name),
    //     format!("sections/{}/default.neoj", kind),
    //     format!("sections/{}/default.neoj", r#type),
    // ];
    // vec![]
}

// fn template_file_exists(path: &PathBuf) -> bool {
//     match path.try_exists() {
//         Ok(exists) => {
//             if exists == true {
//                 true
//             } else {
//                 false
//             }
//         }
//         Err(_) => false,
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    // TODO: Update this for the built-in templates
    #[test]
    #[ignore]
    fn check_make_template_list() {
        let left = vec![
            "sections/title/custom.neoj".to_string(),
            "sections/title/default.neoj".to_string(),
        ];
        let right = make_template_list("basic", "title", "custom");
        assert_eq!(left, right);
    }
}
