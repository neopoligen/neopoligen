use minijinja::Value;
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::nav_item::NavItem;
use neopoligen::nav_item::NavItemType;
use neopoligen::nav_items::NavItems;
use neopoligen::nav_tree::NavTree;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;

#[test]
pub fn basic_load_test() {
    let file_set = FileSet::nav_items1();
    let config = Config::nav_items1();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
    let nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    assert_eq!("folder1-index".to_string(), nav_items.tree[0].page_id);
    assert_eq!(
        "content-alfa".to_string(),
        nav_items.tree[0].children[0].page_id
    );
    assert_eq!(
        "content-bravo".to_string(),
        nav_items.tree[0].children[1].page_id
    );
    assert_eq!(
        "folder1-index".to_string(),
        nav_items.prev_next_items[0].page_id
    );
    assert_eq!(
        "content-alfa".to_string(),
        nav_items.prev_next_items[1].page_id
    );
    assert_eq!(
        "content-bravo".to_string(),
        nav_items.prev_next_items[2].page_id
    );
}

#[test]
pub fn get_next_item_that_exists() {
    let file_set = FileSet::nav_items1();
    let config = Config::nav_items1();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(Value::from("content-alfa"));
    let left = "content-bravo".to_string();
    let right = nav_items.next_item.unwrap().page_id;
    assert_eq!(left, right);
}

#[test]
pub fn get_prev_item_that_exists() {
    let file_set = FileSet::nav_items1();
    let config = Config::nav_items1();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(Value::from("content-alfa"));
    let left = "folder1-index".to_string();
    let right = nav_items.prev_item.unwrap().page_id;
    assert_eq!(left, right);
}

// TODO: Add test for ActiveFolderIndex
