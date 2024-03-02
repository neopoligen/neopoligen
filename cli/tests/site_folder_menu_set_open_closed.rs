mod site_folder_menu_set_open_closed_folders {
    use minijinja::Value;
    use neopoligen::config::Config;
    use neopoligen::file_set::FileSet;
    use neopoligen::site::Site;
    // use pretty_assertions::assert;
    use neopoligen::folder_menu_item::FolderMenuItem;
    use neopoligen::folder_menu_item::FolderMenuItemType;

    #[test]
    pub fn folder_menu_open() {
        let file_set = FileSet::set2();
        let config = Config::site2_config();
        let site = Site::new(&file_set, &config);
        let mut item = FolderMenuItem{
            children: vec![],
            href: None,
            item_type: FolderMenuItemType::OpenDirectory,
            page_id: "local-menu-test-example-title-open-close".to_string(),
            title: Some("Test Folder Item".to_string()),
            folders: vec!["menu".to_string(), "folder".to_string(), "open".to_string()],
        };
        site.folder_menu_set_open_closed_folders(
            &[Value::from("menu-folder-open-switch")],
            &mut item
        );
        assert!(matches!(item.item_type, FolderMenuItemType::OpenDirectory));
    }

    #[test]
    pub fn folder_menu_closed() {
        let file_set = FileSet::set2();
        let config = Config::site2_config();
        let site = Site::new(&file_set, &config);
        let mut item = FolderMenuItem{
            children: vec![],
            href: None,
            item_type: FolderMenuItemType::OpenDirectory,
            page_id: "local-menu-test-example-title-open-close".to_string(),
            title: Some("Test Folder Item".to_string()),
            folders: vec!["non".to_string(), "matching".to_string(), "path".to_string()],
        };
        site.folder_menu_set_open_closed_folders(
            &[Value::from("menu-folder-closed-switch")],
            &mut item
        );
        assert!(matches!(item.item_type, FolderMenuItemType::ClosedDirectory));
    }


}
