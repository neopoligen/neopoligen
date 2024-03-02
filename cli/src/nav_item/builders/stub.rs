use crate::nav_item::NavItem;
use crate::nav_item::NavItemType;

impl NavItem {
    pub fn stub_not_current_file(id: String) -> NavItem {
        NavItem {
            children: vec![],
            folders: vec![],
            is_current_page: false,
            href: None,
            item_type: NavItemType::NotCurrentFile,
            page_id: id,
            path_sort_string: "stub-nav-file-path-sort-string".to_string(),
            title: None,
        }
    }

    pub fn stub_open_folder(id: String) -> NavItem {
        NavItem {
            children: vec![],
            folders: vec![],
            is_current_page: false,
            href: None,
            item_type: NavItemType::OpenFolderTitle,
            page_id: id,
            path_sort_string: "stub-nav-open-folder-sort-string".to_string(),
            title: None,
        }
    }

    pub fn stub_closed_folder(id: String) -> NavItem {
        NavItem {
            children: vec![],
            folders: vec![],
            is_current_page: false,
            href: None,
            item_type: NavItemType::ClosedFolderTitle,
            page_id: id,
            path_sort_string: "stub-nav-closed-folder-sort-string".to_string(),
            title: None,
        }
    }
}
