use minijinja::Value;
use neopoligen::config::Config;
use neopoligen::file_set::FileSet;
use neopoligen::nav_item::NavItem;
use neopoligen::nav_item::NavItemType;
use neopoligen::nav_tree::NavTree;
use neopoligen::site::Site;
use pretty_assertions::assert_eq;
// use std::collections::BTreeMap;

// TODO: Add test for ActiveFolderIndex

#[test]
pub fn load_files_and_folders() {
    let file_set = FileSet::nav_tree_1();
    let config = Config::nav_tree_1();
    let site = Site::new(&file_set, &config);
    let level_1a_content_alfa = NavItem {
        children: vec![],
        href: Some("/en/level-1a-content-alfa/?level-1a-content-alfa".to_string()),
        folders: vec!["level-1a".to_string()],
        is_current_page: false,
        item_type: NavItemType::NotCurrentFile,
        menu_title: Some("Level 1a Content Alfa".to_string()),
        menu_title_link_or_text: Some(format!(
            r#"<a href="{}">{}</a>"#,
            "/en/level-1a-content-alfa/?level-1a-content-alfa".to_string(),
            "Level 1a Content Alfa".to_string()
    )),
        page_id: "level-1a-content-alfa".to_string(),
        path_sort_string: "level-1acontent-alfa.neo".to_string(),
        title: Some("Level 1a Content Alfa".to_string()),
        title_link_or_text: Some(r#"<a href="/en/level-1a-content-alfa/?level-1a-content-alfa">Level 1a Content Alfa</a>"#.to_string()),
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
        menu_title: Some("Level 1a SubLevel 2a Content Bravo".to_string()),
        menu_title_link_or_text: Some(format!(
            r#"<a href="{}">{}</a>"#,
            "/en/level-1a-sub-level-2a-content-bravo/?level-1a-sublevel-2a-content-bravo".to_string(),
            "Level 1a SubLevel 2a Content Bravo".to_string()
            )),
        page_id: "level-1a-sub-level-2a-content-bravo".to_string(),
        path_sort_string: "level-1asub-level-2acontent-bravo.neo".to_string(),
        title: Some("Level 1a SubLevel 2a Content Bravo".to_string()),
        title_link_or_text: Some(
            r#"<a href="/en/level-1a-sub-level-2a-content-bravo/?level-1a-sublevel-2a-content-bravo">Level 1a SubLevel 2a Content Bravo</a>"#.to_string(),
        ),
    };
    let level_1a_sub_level_2a_title = NavItem {
        children: vec![level_1a_sub_level_2a_content_bravo],
        href: Some("/en/level-1a-sub-level-2a-title/?level-1a-sublevel-2a-title".to_string()),
        folders: vec!["level-1a".to_string(), "sub-level-2a".to_string()],
        is_current_page: false,
        item_type: NavItemType::ClosedFolderTitle,
        menu_title: Some("Level 1a SubLevel 2a Title".to_string()),
        menu_title_link_or_text: Some(format!(
            r#"<a href="{}">{}</a>"#,
            "/en/level-1a-sub-level-2a-title/?level-1a-sublevel-2a-title".to_string(),
            "Level 1a SubLevel 2a Title".to_string()
        )),
        page_id: "level-1a-sub-level-2a-title".to_string(),
        path_sort_string: "level-1asub-level-2a_title.neo".to_string(),
        title: Some("Level 1a SubLevel 2a Title".to_string()),
        title_link_or_text: Some(r#"<a href="/en/level-1a-sub-level-2a-title/?level-1a-sublevel-2a-title">Level 1a SubLevel 2a Title</a>"#.to_string()),
    };
    let level_1b_content_charlie = NavItem {
        children: vec![],
        href: Some("/en/level-1b-content-charlie/?level-1b-content-charlie".to_string()),
        folders: vec!["level-1b".to_string()],
        is_current_page: false,
        item_type: NavItemType::NotCurrentFile,
        menu_title: Some("Level 1b Content Charlie".to_string()),
        menu_title_link_or_text: Some(format!(
            r#"<a href="{}">{}</a>"#,
            "/en/level-1b-content-charlie/?level-1b-content-charlie".to_string(),
            "Level 1b Content Charlie".to_string()
            )),
        page_id: "level-1b-content-charlie".to_string(),
        path_sort_string: "level-1bcontent-charlie.neo".to_string(),
        title: Some("Level 1b Content Charlie".to_string()),
        title_link_or_text: Some(r#"<a href="/en/level-1b-content-charlie/?level-1b-content-charlie">Level 1b Content Charlie</a>"#.to_string()),
    };
    let current_page_id = Value::from("top-level-page");
    let files_and_folders = Value::from_serializable::<Vec<Vec<String>>>(&vec![
        vec!["top-level-page".to_string()],
        vec!["level-1a".to_string()],
        vec!["level-1b".to_string()],
    ]);
    let left = NavTree {
        // NOTE: The previous/next stuff is not implemented for
        // these tests
        prev_item: None,
        next_item: None,
        prev_next_order: vec![],
        items: vec![
            NavItem {
                children: vec![],
                href: Some("/en/top-level-page/?top-level-page".to_string()),
                folders: vec![],
                is_current_page: true,
                item_type: NavItemType::CurrentFile,
                menu_title: Some("Top Level Page".to_string()),
                menu_title_link_or_text: Some(format!(r#"{}"#, "Top Level Page".to_string())),
                page_id: "top-level-page".to_string(),
                path_sort_string: "top-level-page.neo".to_string(),
                title: Some("Top Level Page".to_string()),
                title_link_or_text: Some(r#"Top Level Page"#.to_string()),
            },
            NavItem {
                children: vec![level_1a_content_alfa, level_1a_sub_level_2a_title],
                href: Some("/en/level-1a-title/?level-1a-title".to_string()),
                folders: vec!["level-1a".to_string()],
                is_current_page: false,
                item_type: NavItemType::ClosedFolderTitle,
                menu_title: Some("Level 1a Title".to_string()),
                menu_title_link_or_text: Some(format!(
                    r#"<a href="{}">{}</a>"#,
                    "/en/level-1a-title/?level-1a-title".to_string(),
                    "Level 1a Title".to_string()
                )),
                page_id: "level-1a-title".to_string(),
                path_sort_string: "level-1a_title.neo".to_string(),
                title: Some("Level 1a Title".to_string()),
                title_link_or_text: Some(
                    r#"<a href="/en/level-1a-title/?level-1a-title">Level 1a Title</a>"#
                        .to_string(),
                ),
            },
            NavItem {
                children: vec![level_1b_content_charlie],
                href: Some("/en/level-1b-index/?level-1b-index".to_string()),
                folders: vec!["level-1b".to_string()],
                is_current_page: false,
                item_type: NavItemType::ClosedFolderIndex,
                menu_title: Some("Level 1b Index".to_string()),
                menu_title_link_or_text: Some(format!(
                    r#"<a href="{}">{}</a>"#,
                    "/en/level-1b-index/?level-1b-index".to_string(),
                    "Level 1b Index".to_string()
                )),
                page_id: "level-1b-index".to_string(),
                path_sort_string: "level-1b_index.neo".to_string(),
                title: Some("Level 1b Index".to_string()),
                title_link_or_text: Some(
                    r#"<a href="/en/level-1b-index/?level-1b-index">Level 1b Index</a>"#
                        .to_string(),
                ),
            },
        ],
    };

    let right = site.nav_from_files_and_folders(&[current_page_id, files_and_folders]);
    assert_eq!(left, right);
}

// DEPRECTED: This is already tested in the main call to
// .nav_from_files_and_folders(). Delete this is if hasn't
// been restored by May 2024.
//
// #[test]
// pub fn set_current_file() {
//     let file_set = FileSet::nav_tree_2();
//     let config = Config::nav_tree_2();
//     let site = Site::new(&file_set, &config);
//     let current_page_id = Value::from("current-file-target");
//     let files_and_folders = Value::from_serializable::<Vec<Vec<String>>>(&vec![vec![
//         "current-file-target".to_string(),
//     ]]);
//     let left = NavTree {
//         items: vec![NavItem {
//             children: vec![],
//             href: Some("/en/current-file-target/?current-file-target".to_string()),
//             folders: vec![],
//             is_current_page: true,
//             item_type: NavItemType::NotCurrentFile,
//             menu_title: Some("Current File Target".to_string()),
//             menu_title_link_or_text: Some("Current File Target".to_string()),
//             page_id: "current-file-target".to_string(),
//             path_sort_string: "current-file-target.neo".to_string(),
//             title: Some("Current File Target".to_string()),
//             title_link_or_text: Some(format!(r#"{}"#, "Current File Target".to_string())),
//         }],
//     };
//     let mut right = site.nav_from_files_and_folders(&[current_page_id, files_and_folders]);
//     site.set_current_file_for_nav_links(&"current-file-target".to_string(), &mut right);
//     assert_eq!(left, right);
// }
