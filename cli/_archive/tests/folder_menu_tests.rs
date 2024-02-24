#[cfg(test)]
mod folder_menu_tests {
    use minijinja::Value;
    use neopoligen_cli::folder_menu_item::FolderMenuItem;
    use neopoligen_cli::site_v2::SiteV2;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn folder_menu_basic_testc() {
        let site = SiteV2::folder_menu_test_site();
        let page_id = Value::from("menu_alfa");
        let folders = Value::from_serializable(&vec![vec!["level1a"], vec!["level1b"]]);

        let left: Vec<FolderMenuItem> = vec![
            FolderMenuItem {
                page_id: "level1a-index".to_string(),
                is_current_link: false,
                title: Some("level1a index".to_string()),
                href: Some("/en/level1a-index/?level1a-index".to_string()),
                children: vec![
                    FolderMenuItem {
                        page_id: "level1a-file1".to_string(),
                        is_current_link: false,
                        title: Some("level1a file1".to_string()),
                        href: Some("/en/level1a-file1/?level1a-file1".to_string()),
                        children: vec![],
                    },
                    FolderMenuItem {
                        page_id: "level1a-level2a-index".to_string(),
                        is_current_link: false,
                        title: Some("level1a-level2a index".to_string()),
                        href: Some("/en/level1a-level2a-index/?level1a-level2a-index".to_string()),
                        children: vec![],
                    },
                ],
            },
            FolderMenuItem {
                page_id: "level1b-index".to_string(),
                is_current_link: false,
                title: Some("level1b index".to_string()),
                href: Some("/en/level1b-index/?level1b-index".to_string()),
                children: vec![],
            },
        ];
        let right = site.folder_menu(&[page_id, folders]);
        assert_eq!(left, right);
    }
}
