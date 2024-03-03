use minijinja::Value;
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
// use neopoligen::nav_item::NavItem;
// use neopoligen::nav_item::NavItemType;
// use neopoligen::nav_tree::NavTree;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;

// TODO: Add test for ActiveFolderIndex

#[test]
pub fn next_item_basic() {
    let file_set = FileSet::nav_prev_next1();
    let config = Config::nav_prev_next1();
    let site = Site::new(&file_set, &config);
    let current_page_id = Value::from("content-alfa");
    let files_and_folders =
        Value::from_serializable::<Vec<Vec<String>>>(&vec![vec!["level-1a".to_string()]]);
    let nav_links = site.nav_from_files_and_folders_dev(&[current_page_id, files_and_folders]);
    let left = "content-bravo".to_string();
    let right = nav_links.next_item.unwrap().page_id;
    assert_eq!(left, right);
}
