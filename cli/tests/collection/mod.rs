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
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let collection = Collection::new_from_files_and_folders(&site.pages, patterns);
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
    assert_eq!(
        collection.tree[2].children[1].base_type,
        CollectionItemBaseType::IndexFolder
    );
    assert_eq!(
        collection.tree[2].children[1].children[0].base_type,
        CollectionItemBaseType::Page
    );
}

#[test]
pub fn load_page() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![vec!["aabb0010".to_string()]];
    let collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    let left = &"aabb0010".to_string();
    let right = &collection.tree[0].id;
    assert_eq!(left, right);
}

#[test]
pub fn load_page_parent_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![vec!["level-1a".to_string()]];
    let collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    let left = &vec!["level-1a".to_string(), "sub-level-2a".to_string()];
    let right = &collection.tree[0].children[1].children[0].folders;
    assert_eq!(left, right);
}

#[test]
pub fn load_title_folder_parent_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![vec!["level-1a".to_string()]];
    let collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    let left = &vec!["level-1a".to_string(), "sub-level-2a".to_string()];
    let right = &collection.tree[0].children[1].folders;
    assert_eq!(left, right);
}

#[test]
pub fn load_index_folder_parent_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    let left = &vec!["level-1b".to_string()];
    let right = &collection.tree[2].children[0].folders;
    assert_eq!(left, right);
}

#[test]
pub fn load_title_folder() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![vec!["level-1a".to_string()]];
    let collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    let left = &CollectionItemBaseType::TitleFolder;
    let right = &collection.tree[0].base_type;
    assert_eq!(left, right);
}

#[test]
pub fn load_index_folder() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![vec!["level-1b".to_string()]];
    let collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    let left = &CollectionItemBaseType::IndexFolder;
    let right = &collection.tree[0].base_type;
    assert_eq!(left, right);
}

#[test]
pub fn load_collection_children() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![vec!["level-1a".to_string()]];
    let collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    let left = &"aabb0050".to_string();
    let right = &collection.tree[0].children[1].children[0].id;
    assert_eq!(left, right);
}

#[test]
pub fn mark_page_active() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![vec!["level-1a".to_string()]];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
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
    let patterns = vec![vec!["level-1a".to_string()]];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
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
    let patterns = vec![vec!["level-1b".to_string()]];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
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
    let patterns = vec![vec!["level-1a".to_string()]];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    collection.set_active_item(&"aabb0020".to_string());
    let left = &CollectionItemStatus::TitleFolderActive;
    let right = &collection.tree[0].status;
    assert_eq!(left, right);
}

#[test]
pub fn active_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    collection.set_active_item(&"aabb0050".to_string());
    let left = &vec!["level-1a".to_string(), "sub-level-2a".to_string()];
    let right = &collection.active_folders;
    assert_eq!(left, right);
}

#[test]
pub fn mark_title_folder_closed() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    collection.set_active_item(&"aabb0070".to_string());
    let left = &CollectionItemStatus::TitleFolderClosed;
    let right = &collection.tree[1].status;
    assert_eq!(left, right);
}

#[test]
pub fn mark_title_folder_opened() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    collection.set_active_item(&"aabb0050".to_string());
    let left = &CollectionItemStatus::TitleFolderOpened;
    let right = &collection.tree[1].status;
    assert_eq!(left, right);
}

#[test]
pub fn mark_index_folder_closed() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    collection.set_active_item(&"aabb0050".to_string());
    let left = &CollectionItemStatus::IndexFolderClosed;
    let right = &collection.tree[2].status;
    assert_eq!(left, right);
}

#[test]
pub fn mark_index_folder_opened() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    collection.set_active_item(&"aabb0070".to_string());
    let left = &CollectionItemStatus::IndexFolderOpened;
    let right = &collection.tree[2].status;
    assert_eq!(left, right);
}

#[test]
pub fn mark_title_folder_closed_when_no_folders_are_opened() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    collection.set_active_item(&"aabb0010".to_string());
    let left = &CollectionItemStatus::TitleFolderClosed;
    let right = &collection.tree[1].status;
    assert_eq!(left, right);
}

#[test]
pub fn mark_index_folder_closed_when_no_folders_are_opened() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    collection.set_active_item(&"aabb0010".to_string());
    let left = &CollectionItemStatus::IndexFolderClosed;
    let right = &collection.tree[2].status;
    assert_eq!(left, right);
}

#[test]
pub fn ancestors() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    assert_eq!(Vec::<String>::from([]), collection.tree[0].ancestors);
    assert_eq!(Vec::<String>::from([]), collection.tree[1].ancestors);
    assert_eq!(
        Vec::<String>::from(["aabb0020".to_string()]),
        collection.tree[1].children[0].ancestors
    );
    assert_eq!(
        Vec::<String>::from(["aabb0020".to_string()]),
        collection.tree[1].children[1].ancestors
    );
    assert_eq!(
        Vec::<String>::from(["aabb0020".to_string(), "aabb0040".to_string()]),
        collection.tree[1].children[1].children[0].ancestors
    );
    assert_eq!(Vec::<String>::from([]), collection.tree[2].ancestors);
    assert_eq!(
        Vec::<String>::from(["aabb0060".to_string()]),
        collection.tree[2].children[0].ancestors
    );
}

#[test]
// #[ignore]
pub fn prevent_too_many_open_folders() {
    let file_set = FileSet::nav_items2();
    let config = Config::nav_items2();
    let site = Site::new(&file_set, &config);
    let patterns = vec![
        vec!["aabb0010".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ];
    let mut collection = Collection::new_from_files_and_folders(&site.pages, patterns);
    collection.set_active_item(&"aabb0030".to_string());
    assert_eq!(
        CollectionItemStatus::TitleFolderOpened,
        collection.tree[1].status
    );
    assert_eq!(
        CollectionItemStatus::PageActive,
        collection.tree[1].children[0].status
    );

    assert_eq!(
        CollectionItemStatus::TitleFolderClosed,
        collection.tree[1].children[1].status
    );
}
