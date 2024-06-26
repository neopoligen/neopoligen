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
pub mod site_page_template;
pub mod site_page_title;
pub mod template_ilink;

use minijinja::Value;
use neopoligengine::config::Config;
use neopoligengine::file_set::FileSet;
use neopoligengine::site::Site;
use pretty_assertions::assert_eq;

#[test]
pub fn get_collection_subtree() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let id = Value::from("aabb0010".to_string());
    let patterns = Value::from_serialize::<Vec<Vec<String>>>(vec![vec!["level-1a".to_string()]]);
    let site_collection = site.collection_from_files_and_folders(&[id, patterns]);
    let sub_tree_request_id = Value::from("aabb0020".to_string());
    let original_tree = Value::from_serialize(&site_collection);
    let sub_tree = site.get_subtree(&[sub_tree_request_id, original_tree]);
    let left = &"aabb0030".to_string();
    let right = &sub_tree[0].id;
    assert_eq!(left, right);
}

#[test]
pub fn load_images_from_file_set() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = &"/images/root-level-image.png".to_string();
    let right = &site.images[0].raw_href;
    assert_eq!(left, right);
}

#[test]
#[ignore]
pub fn image_path_from_name_with_extension_in_top_dir() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = &"/images/root-level-image.png".to_string();
    let right = &site
        .image(&[Value::from("root-level-image.png".to_string())])
        .unwrap()
        .raw_href;
    assert_eq!(left, right);
}

#[test]
pub fn image_path_from_name_with_extension_in_sub_dir() {
    let file_set = FileSet::set1();
    let config = Config::set1();
    let site = Site::new(&file_set, &config);
    let left = &"/images/sub-folder/sub-folder-image.png".to_string();
    let right = &site
        .image(&[Value::from("sub-folder-image.png".to_string())])
        .unwrap()
        .raw_href;
    assert_eq!(left, right);
}

// // Deprecated
// #[test]
// pub fn does_template_exists_no() {
//     let file_set = FileSet::set1();
//     let config = Config::set1();
//     let site = Site::new(&file_set, &config);
//     let left = &"no".to_string();
//     let right = &site.does_template_exist(&[Value::from(
//         "path/to/template/that/does/not/exist.jinaj".to_string(),
//     )]);
//     assert_eq!(left, right);
// }

// // Deprecated
// #[test]
// pub fn does_template_exists_yes() {
//     let file_set = FileSet::set1();
//     let config = Config::set1();
//     let site = Site::new(&file_set, &config);
//     let left = &"yes".to_string();
//     let right = &site.does_template_exist(&[Value::from("custom/template/path.neojinja".to_string())]);
//     assert_eq!(left, right);
// }
