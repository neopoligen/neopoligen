use minijinja::Value;
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::nav_item::NavItemType;
use neopoligen::nav_items::NavItems;
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
    nav_items.set_current_page(&Value::from("content-alfa"));
    let left = "content-bravo".to_string();
    let right = nav_items.next_item.unwrap().page_id;
    assert_eq!(left, right);
}

#[test]
pub fn get_next_item_that_does_not_exist() {
    let file_set = FileSet::nav_items1();
    let config = Config::nav_items1();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("content-bravo"));
    let left = None;
    let right = nav_items.next_item;
    assert_eq!(left, right);
}

#[test]
pub fn get_prev_item_that_exists() {
    let file_set = FileSet::nav_items1();
    let config = Config::nav_items1();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("content-alfa"));
    let left = "folder1-index".to_string();
    let right = nav_items.prev_item.unwrap().page_id;
    assert_eq!(left, right);
}

#[test]
pub fn get_prev_item_that_does_not_exist() {
    let file_set = FileSet::nav_items1();
    let config = Config::nav_items1();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("folder1-index"));
    let left = None;
    let right = nav_items.prev_item;
    assert_eq!(left, right);
}

#[test]
pub fn set_current_file() {
    let file_set = FileSet::nav_items1();
    let config = Config::nav_items1();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("content-alfa"));
    assert_eq!(
        nav_items.tree[0].children[0].item_type,
        NavItemType::CurrentFile
    );
}

#[test]
pub fn check_not_current_file() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
        vec!["aabb0010"],
        vec!["level-1a"],
        vec!["level-1b"],
    ]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("content-alfa"));
    assert_eq!(nav_items.tree[0].item_type, NavItemType::NotCurrentFile);
    assert_eq!(
        nav_items.tree[2].children[0].item_type,
        NavItemType::NotCurrentFile
    );
}

#[test]
pub fn set_top_level_current_file() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
        vec!["aabb0010"],
        vec!["level-1a"],
        vec!["level-1b"],
    ]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("aabb0010"));
    assert_eq!(nav_items.tree[0].item_type, NavItemType::CurrentFile);
}

#[test]
pub fn check_active_folder_index() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1b"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("aabb0060"));
    assert_eq!(nav_items.tree[0].item_type, NavItemType::ActiveFolderIndex);
}

#[test]
pub fn check_closed_folder_index() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1b"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("content-alfa"));
    assert_eq!(nav_items.tree[0].item_type, NavItemType::ClosedFolderIndex);
}

#[test]
pub fn check_closed_folder_title() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns =
        Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"], vec!["level-1b"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("content-charlie"));
    let left = NavItemType::ClosedFolderTitle;
    let right = nav_items.tree[0].item_type.clone();
    assert_eq!(left, right);
}

#[test]
pub fn check_opened_folder_title() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns =
        Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"], vec!["level-1b"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("aabb0030"));
    let left = NavItemType::OpenedFolderTitle;
    let right = nav_items.tree[0].item_type.clone();
    assert_eq!(left, right);
}

#[test]
pub fn check_opened_folder_index() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns =
        Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"], vec!["level-1b"]]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("aabb0070"));
    let left = NavItemType::OpenedFolderIndex;
    let right = nav_items.tree[1].item_type.clone();
    assert_eq!(left, right);
}

#[test]
pub fn prev_next_skips_title_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
        vec!["aabb0010"],
        vec!["level-1a"],
        vec!["level-1b"],
    ]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("aabb0010"));
    let left = String::from("aabb0030");
    let right = nav_items.prev_next_items[1].page_id.clone();
    assert_eq!(left, right);
}

#[test]
pub fn get_parent_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
        vec!["aabb0010"],
        vec!["level-1a"],
        vec!["level-1b"],
    ]);
    let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
    nav_items.set_current_page(&Value::from("aabb0010"));
    let left = String::from("aabb0030");
    let right = nav_items.prev_next_items[1].page_id.clone();
    assert_eq!(left, right);
}
