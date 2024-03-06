pub mod integration;
pub mod parsing_basic;
pub mod site_link_or_titile;
pub mod site_page_folders;
pub mod site_page_href;
pub mod site_page_href_title;
pub mod site_page_ids;
pub mod site_page_main_body;
pub mod site_page_menu_title;
pub mod site_page_output_path;
pub mod site_page_path_parts;
pub mod site_page_place_section;
pub mod site_page_source_path;
pub mod site_page_status;
pub mod site_page_template;
pub mod site_page_title;
pub mod site_page_type;
pub mod template_ilink;

use minijinja::Value;
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;

#[test]
pub fn get_collection_subtree() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let id = Value::from("aabb0010".to_string());
    let patterns =
        Value::from_serializable::<Vec<Vec<String>>>(&vec![vec!["level-1a".to_string()]]);
    let site_collection = site.collection_from_files_and_folders(&[id, patterns]);
    let sub_tree_request_id = Value::from("aabb0020".to_string());
    let original_tree = Value::from_serializable(&site_collection);
    let sub_tree = site.get_subtree(&[sub_tree_request_id, original_tree]);
    dbg!(sub_tree);

    // let left = Some(format!(r#"Lorem Ipsum"#));
    // let right = site.ilink(&[
    //     Value::from("ttss0010"),
    //     Value::from("ttss0010"),
    //     Value::from("Lorem Ipsum"),
    // ]);
    // assert_eq!(left, right);
}
