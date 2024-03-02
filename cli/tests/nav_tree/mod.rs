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
    let current_page_id = Value::from("top-level-page");
    let files_and_folders = Value::from_serializable::<Vec<Vec<String>>>(&vec![
        vec!["top-level-page".to_string()],
        vec!["level-1a".to_string()],
    ]);
    let left = NavTree {
        items: vec![
            NavItem {
                children: vec![],
                href: Some("/en/top-level-page/?top-level-page".to_string()),
                folders: vec![],
                is_current_page: false,
                item_type: NavItemType::File,
                page_id: "top-level-page".to_string(),
                path_sort_string: "top-level-page.neo".to_string(),
                title: Some("Top Level Page".to_string()),
            },
            NavItem {
                children: vec![
                    NavItem {
                        children: vec![],
                        href: Some("/en/level-1a-content-alfa/?level-1a-content-alfa".to_string()),
                        folders: vec!["level-1a".to_string()],
                        is_current_page: false,
                        item_type: NavItemType::File,
                        page_id: "level-1a-content-alfa".to_string(),
                        path_sort_string: "level-1acontent-alfa.neo".to_string(),
                        title: Some("Level 1a Content Alfa".to_string()),
                    },
                    NavItem {
                        children: vec![],
                        href: Some(
                            "/en/level-1a-content-bravo/?level-1a-content-bravo".to_string(),
                        ),
                        folders: vec!["level-1a".to_string()],
                        is_current_page: false,
                        item_type: NavItemType::File,
                        page_id: "level-1a-content-bravo".to_string(),
                        path_sort_string: "level-1acontent-bravo.neo".to_string(),
                        title: Some("Level 1a Content Bravo".to_string()),
                    },
                    NavItem {
                        children: vec![NavItem {
                            children: vec![],
                            href: Some(
                                "/en/level-1a-sub-level-2a-content-echo/?level-1a-sublevel-2a-content-echo".to_string(),
                            ),
                            folders: vec!["level-1a".to_string(), "sub-level-2a".to_string()],
                            is_current_page: false,
                            item_type: NavItemType::File,
                            page_id: "level-1a-sub-level-2a-content-echo".to_string(),
                            path_sort_string: "level-1asub-level-2acontent-echo.neo".to_string(),
                            title: Some("Level 1a SubLevel 2a Content Echo".to_string()),
                        }],
                        href: Some(
                            "/en/level-1a-sub-level-2a-index/?level-1a-sublevel-2a-index"
                                .to_string(),
                        ),
                        folders: vec!["level-1a".to_string(), "sub-level-2a".to_string()],
                        is_current_page: false,
                        item_type: NavItemType::ClosedFolderTitle,
                        page_id: "level-1a-sub-level-2a-index".to_string(),
                        path_sort_string: "level-1asub-level-2a_title.neo".to_string(),
                        title: Some("Level 1a SubLevel 2a Index".to_string()),
                    },
                ],
                href: Some("/en/level-1a-index/?level-1a-index".to_string()),
                folders: vec!["level-1a".to_string()],
                is_current_page: false,
                item_type: NavItemType::ClosedFolderTitle,
                page_id: "level-1a-index".to_string(),
                path_sort_string: "level-1a_title.neo".to_string(),
                title: Some("Level 1a Index".to_string()),
            },
        ],
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
        item_type: NavItemType::OpenFolderTitle,
        page_id: "local-menu-test-example-title-open-close".to_string(),
        title: Some("Test Folder Item".to_string()),
        folders: vec!["menu".to_string(), "folder".to_string(), "open".to_string()],
        path_sort_string: "skipping-sort-for-this-test-a".to_string(),
        is_current_page: false,
    };
    site.folder_menu_set_open_closed_folders(&[Value::from("menu-folder-open-switch")], &mut item);
    assert!(matches!(item.item_type, NavItemType::OpenFolderTitle));
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
        item_type: NavItemType::OpenFolderTitle,
        page_id: "local-menu-test-example-title-open-close".to_string(),
        title: Some("Test Folder Item".to_string()),
        folders: vec![
            "non".to_string(),
            "matching".to_string(),
            "path".to_string(),
        ],
        path_sort_string: "skipping-sort-for-this-test-b".to_string(),
        is_current_page: false,
    };
    site.folder_menu_set_open_closed_folders(
        &[Value::from("menu-folder-closed-switch")],
        &mut item,
    );
    assert!(matches!(item.item_type, NavItemType::ClosedFolderTitle));
}
