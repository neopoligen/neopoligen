use minijinja::Value;
use neopoligen::collection::{Collection, CollectionItemBaseType, CollectionItemStatus};
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;

#[test]
pub fn item_reference() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
        vec!["aabb0010"],
        vec!["level-1a"],
        vec!["level-1b"],
    ]);
    let collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    assert_eq!(&collection.tree[0].id, &"aabb0010".to_string());
    assert_eq!(&collection.tree[1].id, &"aabb0020".to_string());
    assert_eq!(&collection.tree[1].children[0].id, &"aabb0030".to_string());
    assert_eq!(&collection.tree[1].children[1].id, &"aabb0040".to_string());
    assert_eq!(
        &collection.tree[1].children[1].children[0].id,
        &"aabb0050".to_string()
    );
    assert_eq!(&collection.tree[2].id, &"aabb0060".to_string());
    assert_eq!(&collection.tree[2].children[0].id, &"aabb0070".to_string());

    assert_eq!(collection.tree[0].base_type, CollectionItemBaseType::Page);
    assert_eq!(
        collection.tree[1].base_type,
        CollectionItemBaseType::TitleFolder
    );
    assert_eq!(
        collection.tree[1].children[0].base_type,
        CollectionItemBaseType::Page
    );
    assert_eq!(
        collection.tree[1].children[1].base_type,
        CollectionItemBaseType::TitleFolder
    );
    assert_eq!(
        collection.tree[1].children[1].children[0].base_type,
        CollectionItemBaseType::Page
    );
    assert_eq!(
        collection.tree[2].base_type,
        CollectionItemBaseType::IndexFolder
    );
    assert_eq!(
        collection.tree[2].children[0].base_type,
        CollectionItemBaseType::Page
    );
}

#[test]
pub fn load_page() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["aabb0010"]]);
    let collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    let left = &"aabb0010".to_string();
    let right = &collection.tree[0].id;
    assert_eq!(left, right);
}

#[test]
pub fn load_page_parent_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
    let collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    let left = &vec!["level-1a".to_string(), "sub-level-2a".to_string()];
    let right = &collection.tree[0].children[1].children[0].folders;
    assert_eq!(left, right);
}

#[test]
pub fn load_title_folder_parent_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
    let collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    let left = &vec!["level-1a".to_string(), "sub-level-2a".to_string()];
    let right = &collection.tree[0].children[1].folders;
    assert_eq!(left, right);
}

#[test]
pub fn load_index_folder_parent_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
        vec!["aabb0010"],
        vec!["level-1a"],
        vec!["level-1b"],
    ]);
    let collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    let left = &vec!["level-1b".to_string()];
    let right = &collection.tree[2].children[0].folders;
    assert_eq!(left, right);
}

#[test]
pub fn load_title_folder() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
    let collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    let left = &CollectionItemBaseType::TitleFolder;
    let right = &collection.tree[0].base_type;
    assert_eq!(left, right);
}

#[test]
pub fn load_index_folder() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1b"]]);
    let collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    let left = &CollectionItemBaseType::IndexFolder;
    let right = &collection.tree[0].base_type;
    assert_eq!(left, right);
}

#[test]
pub fn load_collection_children() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
    let collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    let left = &"aabb0050".to_string();
    let right = &collection.tree[0].children[1].children[0].id;
    assert_eq!(left, right);
}

#[test]
pub fn mark_page_active() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
    let mut collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    collection.set_active_item(&"aabb0050".to_string());
    let left = &CollectionItemStatus::PageActive;
    let right = &collection.tree[0].children[1].children[0].status;
    assert_eq!(left, right);
}

#[test]
pub fn mark_page_inactive() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
    let mut collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    collection.set_active_item(&"aabb0050".to_string());
    let left = &CollectionItemStatus::PageInactive;
    let right = &collection.tree[0].children[0].status;
    assert_eq!(left, right);
}

#[test]
pub fn mark_index_folder_active() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1b"]]);
    let mut collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    collection.set_active_item(&"aabb0060".to_string());
    let left = &CollectionItemStatus::IndexFolderActive;
    let right = &collection.tree[0].status;
    assert_eq!(left, right);
}

#[test]
pub fn mark_title_folder_active() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
    let mut collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    collection.set_active_item(&"aabb0020".to_string());
    let left = &CollectionItemStatus::TitleFolderActive;
    let right = &collection.tree[0].status;
    assert_eq!(left, right);
}

#[test]
#[ignore]
pub fn mark_title_folder_closed() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
        vec!["aabb0010"],
        vec!["level-1a"],
        vec!["level-1b"],
    ]);
    let mut collection = Collection::new_from_files_and_folders(&site.pages, &[patterns]);
    collection.set_active_item(&"aabb0010".to_string());
    let left = &CollectionItemStatus::TitleFolderClosed;
    let right = &collection.tree[1].status;
    assert_eq!(left, right);
}

// #[test]
// pub fn get_next_item_that_exists() {
//     let file_set = FileSet::nav_items1();
//     let config = Config::nav_items1();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("content-alfa"));
//     let left = "content-bravo".to_string();
//     let right = nav_items.next_item.unwrap().page_id;
//     assert_eq!(left, right);
// }

// #[test]
// pub fn get_next_item_that_does_not_exist() {
//     let file_set = FileSet::nav_items1();
//     let config = Config::nav_items1();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("content-bravo"));
//     let left = None;
//     let right = nav_items.next_item;
//     assert_eq!(left, right);
// }

// #[test]
// pub fn get_prev_item_that_exists() {
//     let file_set = FileSet::nav_items1();
//     let config = Config::nav_items1();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("content-alfa"));
//     let left = "folder1-index".to_string();
//     let right = nav_items.prev_item.unwrap().page_id;
//     assert_eq!(left, right);
// }

// #[test]
// pub fn get_prev_item_that_does_not_exist() {
//     let file_set = FileSet::nav_items1();
//     let config = Config::nav_items1();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["folder1"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("folder1-index"));
//     let left = None;
//     let right = nav_items.prev_item;
//     assert_eq!(left, right);
// }

// #[test]
// pub fn file_not_current() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
//         vec!["aabb0010"],
//         vec!["level-1a"],
//         vec!["level-1b"],
//     ]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("content-alfa"));
//     assert_eq!(nav_items.tree[0].item_type, NavItemBaseType::FileNotCurrent);
//     assert_eq!(
//         nav_items.tree[2].children[0].item_type,
//         NavItemBaseType::FileNotCurrent
//     );
// }

// #[test]
// pub fn set_top_level_current_file() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
//         vec!["aabb0010"],
//         vec!["level-1a"],
//         vec!["level-1b"],
//     ]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("aabb0010"));
//     assert_eq!(nav_items.tree[0].item_type, NavItemBaseType::FileCurrent);
// }

// #[test]
// pub fn index_folder_active() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1b"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("aabb0060"));
//     assert_eq!(nav_items.tree[0].item_type, NavItemBaseType::IndexFolderActive);
// }

// #[test]
// pub fn index_folder_closed() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1b"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("content-alfa"));
//     assert_eq!(nav_items.tree[0].item_type, NavItemBaseType::IndexFolderClosed);
// }

// #[test]
// pub fn title_folder_closed() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns =
//         Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"], vec!["level-1b"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("content-charlie"));
//     let left = NavItemBaseType::TitleFolderClosed;
//     let right = nav_items.tree[0].item_type.clone();
//     assert_eq!(left, right);
// }

// #[test]
// pub fn title_folder_opened() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns =
//         Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"], vec!["level-1b"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("aabb0030"));
//     let left = NavItemBaseType::TitleFolderOpened;
//     let right = nav_items.tree[0].item_type.clone();
//     assert_eq!(left, right);
// }

// #[test]
// pub fn title_folder_active() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns =
//         Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"], vec!["level-1b"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("aabb0020"));
//     let left = NavItemBaseType::TitleFolderActive;
//     let right = nav_items.tree[0].item_type.clone();
//     assert_eq!(left, right);
// }

// #[test]
// pub fn index_folder_opened() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns =
//         Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"], vec!["level-1b"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("aabb0070"));
//     let left = NavItemBaseType::IndexFolderOpened;
//     let right = nav_items.tree[1].item_type.clone();
//     assert_eq!(left, right);
// }

// #[test]
// pub fn prev_next_skips_title_folders() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![
//         vec!["aabb0010"],
//         vec!["level-1a"],
//         vec!["level-1b"],
//     ]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("aabb0010"));
//     let left = String::from("aabb0030");
//     let right = nav_items.prev_next_items[1].page_id.clone();
//     assert_eq!(left, right);
// }

// #[test]
// pub fn check_parent_ids() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
//     let nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     let left = String::from("aabb0020");
//     let right = nav_items.tree[0].children[0].parent_ids[0].clone();
//     assert_eq!(left, right);
// }

// #[test]
// pub fn check_parent_ids_second_level() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
//     let nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     let left = vec!["aabb0020".to_string(), "aabb0040".to_string()];
//     let right = nav_items.tree[0].children[1].children[0].parent_ids.clone();
//     assert_eq!(left, right);
// }

// #[test]
// pub fn current_breadcrumbs() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
//     let mut nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     nav_items.set_current_page(&Value::from("aabb0050"));
//     let left = vec!["aabb0020".to_string(), "aabb0040".to_string()];
//     let right = nav_items.current_item.unwrap().parent_ids.clone();
//     assert_eq!(left, right);
// }

// #[test]
// pub fn nav_tree_ids_from() {
//     let file_set = FileSet::nav_items2();
//     let config = Config::nav_items2();
//     let site = Site::new(&file_set, &config);
//     let patterns = Value::from_serializable::<Vec<Vec<&str>>>(&vec![vec!["level-1a"]]);
//     let nav_items = NavItems::new_from_files_and_folders(&site, &patterns);
//     let left = vec![
//         NavId {
//             page_id: "aabb0030".to_string(),
//             base_type: NavIdBaseType::File,
//             children: vec![],
//         },
//         NavId {
//             page_id: "aabb0040".to_string(),
//             base_type: NavIdBaseType::File,
//             children: vec![NavId {
//                 page_id: "aabb0050".to_string(),
//                 base_type: NavIdBaseType::File,
//                 children: vec![],
//             }],
//         },
//     ];
//     let right = nav_items.tree_items_from(&[Value::from("aabb0020".to_string())]);
//     assert_eq!(left, right);
// }
