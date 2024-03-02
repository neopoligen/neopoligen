use minijinja::Value;
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::nav_item::NavItem;
use neopoligen::nav_item::NavItemType;
use neopoligen::nav_tree::NavTree;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;

#[test]
// #[ignore]
pub fn load_files_and_folders() {
    let file_set = FileSet::nav_tree_1();
    let config = Config::nav_tree_1();
    let site = Site::new(&file_set, &config);
    let top_level_page = NavItem {
        children: vec![],
        href: Some("/en/top-level-page/?top-level-page".to_string()),
        folders: vec![],
        is_current_page: false,
        item_type: NavItemType::NotCurrentFile,
        page_id: "top-level-page".to_string(),
        path_sort_string: "top-level-page.neo".to_string(),
        title: Some("Top Level Page".to_string()),
        menu_title: Some("Top Level Page".to_string()),
        // menu_title_link_or_text: Some("Top Level Page".to_string()),
    };
    let level_1a_content_alfa = NavItem {
        children: vec![],
        href: Some("/en/level-1a-content-alfa/?level-1a-content-alfa".to_string()),
        folders: vec!["level-1a".to_string()],
        is_current_page: false,
        item_type: NavItemType::NotCurrentFile,
        page_id: "level-1a-content-alfa".to_string(),
        path_sort_string: "level-1acontent-alfa.neo".to_string(),
        title: Some("Level 1a Content Alfa".to_string()),
        menu_title: Some("Level 1a Content Alfa".to_string()),
        // menu_title_link_or_text: Some("Level 1a Content Alfa".to_string()),
    };
    let level_1a_sub_level_2a_content_bravo = NavItem {
        children: vec![],
        href: Some(
            "/en/level-1a-sub-level-2a-content-bravo/?level-1a-sublevel-2a-content-bravo"
                .to_string(),
        ),
        folders: vec!["level-1a".to_string(), "sub-level-2a".to_string()],
        is_current_page: false,
        item_type: NavItemType::NotCurrentFile,
        page_id: "level-1a-sub-level-2a-content-bravo".to_string(),
        path_sort_string: "level-1asub-level-2acontent-bravo.neo".to_string(),
        title: Some("Level 1a SubLevel 2a Content Bravo".to_string()),
        menu_title: Some("Level 1a SubLevel 2a Content Bravo".to_string()),
        // menu_title_link_or_text: Some("Level 1a SubLevel 2a Content Bravo".to_string()),
    };
    let level_1a_sub_level_2a_title = NavItem {
        children: vec![level_1a_sub_level_2a_content_bravo],
        href: Some("/en/level-1a-sub-level-2a-title/?level-1a-sublevel-2a-title".to_string()),
        folders: vec!["level-1a".to_string(), "sub-level-2a".to_string()],
        is_current_page: false,
        item_type: NavItemType::ClosedFolderTitle,
        page_id: "level-1a-sub-level-2a-title".to_string(),
        path_sort_string: "level-1asub-level-2a_title.neo".to_string(),
        title: Some("Level 1a SubLevel 2a Title".to_string()),
        menu_title: Some("Level 1a SubLevel 2a Title".to_string()),
        // menu_title_link_or_text: Some("Level 1a SubLevel 2a Title".to_string()),
    };
    let level_1a_title = NavItem {
        children: vec![level_1a_content_alfa, level_1a_sub_level_2a_title],
        href: Some("/en/level-1a-title/?level-1a-title".to_string()),
        folders: vec!["level-1a".to_string()],
        is_current_page: false,
        item_type: NavItemType::ClosedFolderTitle,
        page_id: "level-1a-title".to_string(),
        path_sort_string: "level-1a_title.neo".to_string(),
        title: Some("Level 1a Title".to_string()),
        menu_title: Some("Level 1a Title".to_string()),
        // menu_title_link_or_text: Some("Level 1a Title".to_string()),
    };
    let level_1b_content_charlie = NavItem {
        children: vec![],
        href: Some("/en/level-1b-content-charlie/?level-1b-content-charlie".to_string()),
        folders: vec!["level-1b".to_string()],
        is_current_page: false,
        item_type: NavItemType::NotCurrentFile,
        page_id: "level-1b-content-charlie".to_string(),
        path_sort_string: "level-1bcontent-charlie.neo".to_string(),
        title: Some("Level 1b Content Charlie".to_string()),
        menu_title: Some("Level 1b Content Charlie".to_string()),
        // menu_title_link_or_text: Some("Level 1b Content Charlie".to_string()),
    };
    let level_1b_index = NavItem {
        children: vec![level_1b_content_charlie],
        href: Some("/en/level-1b-index/?level-1b-index".to_string()),
        folders: vec!["level-1b".to_string()],
        is_current_page: false,
        item_type: NavItemType::ClosedFolderIndex,
        page_id: "level-1b-index".to_string(),
        path_sort_string: "level-1b_index.neo".to_string(),
        title: Some("Level 1b Index".to_string()),
        menu_title: Some("Level 1b Index".to_string()),
        // menu_title_link_or_text: Some("Level 1b Index".to_string()),
    };
    let current_page_id = Value::from("top-level-page");
    let files_and_folders = Value::from_serializable::<Vec<Vec<String>>>(&vec![
        vec!["top-level-page".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ]);
    let left = NavTree {
        items: vec![top_level_page, level_1a_title, level_1b_index],
    };

    let right = site.nav_from_files_and_folders(&[current_page_id, files_and_folders]);
    assert_eq!(left, right);
}

// These are the origial folders ones that should be checked to
// see what needs to be pulled in

#[test]
#[ignore]
pub fn folder_menu_open() {
    let file_set = FileSet::set2();
    let config = Config::site2_config();
    let site = Site::new(&file_set, &config);
    let mut item = NavItem {
        children: vec![],
        href: None,
        item_type: NavItemType::OpenedFolderTitle,
        page_id: "local-menu-test-example-title-open-close".to_string(),
        folders: vec!["menu".to_string(), "folder".to_string(), "open".to_string()],
        path_sort_string: "skipping-sort-for-this-test-a".to_string(),
        is_current_page: false,
        title: Some("Test Folder Item".to_string()),
        menu_title: Some("Test Folder Item".to_string()),
        // menu_title_link_or_text: Some("Test Folder Item".to_string()),
    };
    site.folder_menu_set_open_closed_folders(&[Value::from("menu-folder-open-switch")], &mut item);
    assert!(matches!(item.item_type, NavItemType::OpenedFolderTitle));
}

#[test]
#[ignore]
pub fn folder_menu_closed() {
    let file_set = FileSet::set2();
    let config = Config::site2_config();
    let site = Site::new(&file_set, &config);
    let mut item = NavItem {
        children: vec![],
        href: None,
        item_type: NavItemType::OpenedFolderTitle,
        page_id: "local-menu-test-example-title-open-close".to_string(),
        folders: vec![
            "non".to_string(),
            "matching".to_string(),
            "path".to_string(),
        ],
        path_sort_string: "skipping-sort-for-this-test-b".to_string(),
        is_current_page: false,
        title: Some("Test Folder Item".to_string()),
        menu_title: Some("Test Folder Item".to_string()),
        // menu_title_link_or_text: Some("Test Folder Item".to_string()),
    };
    site.folder_menu_set_open_closed_folders(
        &[Value::from("menu-folder-closed-switch")],
        &mut item,
    );
    assert!(matches!(item.item_type, NavItemType::ClosedFolderTitle));
}
