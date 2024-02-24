// This was an initila run for the page_id based menus.
// It took a while to figure out how to do them
// so these tests don't currently match

#[cfg(test)]
mod menu_items_tests {

    // use minijinja::Value;
    // use neopoligen_cli::menu_item::*;
    // use neopoligen_cli::site_v2::SiteV2;
    // use pretty_assertions::assert_eq;
    // use serde_json::Value as SerdeValue;
    // // use std::collections::BTreeMap;

    // #[test]
    // pub fn menu_items_basci_test() {
    //     let site = SiteV2::menu_test_site();
    //     let current_page = Value::from("menu_bravo");
    //     let payload_text = r#"[
    //     { "page_id": "menu_alfa" },
    //     { "page_id": "menu_bravo" },
    //     { "page_id": "menu_charlie" },
    //     { "page_id": "menu_delta" }
    //     ]"#;
    //     let payload =
    //         Value::from_serializable(&serde_json::from_str::<SerdeValue>(payload_text).unwrap());
    //     let left = vec![
    //         MenuItem {
    //             page_id: "menu_alfa".to_string(),
    //             items: vec![
    //                 MenuItem {
    //                     page_id: "menu_echo".to_string(),
    //                     items: vec![],
    //                 },
    //                 MenuItem {
    //                     page_id: "menu_foxtrot".to_string(),
    //                     items: vec![
    //                         MenuItem {
    //                             page_id: "menu_mike".to_string(),
    //                             items: vec![
    //                                 MenuItem {
    //                                     page_id: "menu_lima".to_string(),
    //                                     items: vec![],
    //                                 },
    //                                 MenuItem {
    //                                     page_id: "menu_november".to_string(),
    //                                     items: vec![],
    //                                 },
    //                             ],
    //                         },
    //                         MenuItem {
    //                             page_id: "menu_oscar".to_string(),
    //                             items: vec![
    //                                 MenuItem {
    //                                     page_id: "menu_papa".to_string(),
    //                                     items: vec![],
    //                                 },
    //                                 MenuItem {
    //                                     page_id: "menu_quebec".to_string(),
    //                                     items: vec![],
    //                                 },
    //                             ],
    //                         },
    //                     ],
    //                 },
    //                 MenuItem {
    //                     page_id: "menu_golf".to_string(),
    //                     items: vec![],
    //                 },
    //                 MenuItem {
    //                     page_id: "menu_hotel".to_string(),
    //                     items: vec![],
    //                 },
    //             ],
    //         },
    //         MenuItem {
    //             page_id: "menu_bravo".to_string(),
    //             items: vec![
    //                 MenuItem {
    //                     page_id: "menu_india".to_string(),
    //                     items: vec![],
    //                 },
    //                 MenuItem {
    //                     page_id: "menu_juliett".to_string(),
    //                     items: vec![],
    //                 },
    //                 MenuItem {
    //                     page_id: "menu_kilo".to_string(),
    //                     items: vec![],
    //                 },
    //             ],
    //         },
    //         MenuItem {
    //             page_id: "menu_charlie".to_string(),
    //             items: vec![],
    //         },
    //         MenuItem {
    //             page_id: "menu_delta".to_string(),
    //             items: vec![],
    //         },
    //     ];
    //     let right = site.menu_items(&[current_page, payload]);
    //     assert_eq!(left, right);
    // }

    // #[test]
    // pub fn menu_links_basic() {
    //     let site = SiteV2::menu_test_site();
    //     let current_page = Value::from("menu_bravo");
    //     let mut p1: BTreeMap<String, String> = BTreeMap::new();
    //     p1.insert("page_id".to_string(), "menu_alfa".to_string());
    //     let mut p2: BTreeMap<String, String> = BTreeMap::new();
    //     p2.insert("page_id".to_string(), "menu_bravo".to_string());
    //     let mut p3: BTreeMap<String, String> = BTreeMap::new();
    //     p3.insert("page_id".to_string(), "menu_charlie".to_string());
    //     let mut p4: BTreeMap<String, String> = BTreeMap::new();
    //     p4.insert("page_id".to_string(), "menu_delta".to_string());
    //     let links_arg = Value::from_serializable(&vec![p1, p2, p3, p4]);

    //         let left = Some(
    //             r#"<ol>
    // <li>
    // <a href="/en/menu_alfa/?menu-page:-alfa">Menu Page: alfa</a>
    // </li>
    // <li>
    // Menu Page: bravo
    // </li>
    // <li>
    // <a href="/en/menu_charlie/?menu-page:-charlie">Menu Page: charlie</a>
    // </li>
    // <li>
    // <a href="/en/menu_delta/?menu-page:-delta">Menu Page: delta</a>
    // </li>
    //         </ol>"#
    //                 .replace(" ", "")
    //                 .replace("\n", "")
    //                 .to_string(),
    //         )
    //         .unwrap();
    //         let right = site
    //             .menu_items(&[current_page, links_arg])
    //             .unwrap()
    //             .replace(" ", "")
    //             .replace("\n", "");
    //         assert_eq!(left, right);
    //     }
}
