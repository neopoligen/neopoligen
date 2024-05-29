pub mod mocks;

#[derive(Debug)]
pub struct PageFilterOrSet {
    pub and_groups: Vec<PageFilterAndGroup>,
}

impl PageFilterOrSet {
    pub fn new() -> PageFilterOrSet {
        PageFilterOrSet { and_groups: vec![] }
    }
}

#[derive(Debug)]
pub struct PageFilterAndGroup {
    pub filters: Vec<PageFilter>,
}

#[derive(Debug, PartialEq)]
pub enum PageFilter {
    Status { exclude: bool, value: String },
}

impl PageFilter {
    pub fn parse(source: &str) -> Option<PageFilter> {
        if let Some(parts) = source.split_once(":") {
            let (exclude, value) = match parts.1.strip_prefix("!") {
                Some(value) => (true, value.trim().to_string()),
                None => (false, parts.1.trim().to_string()),
            };
            match parts.0 {
                "status" => Some(PageFilter::Status { exclude, value }),
                _ => None,
            }
        } else {
            None
        }
    }
}

// #[derive(Debug)]
// pub enum PageFilterType {
//     RootFolder,
//     Folder,
//     Status,
//     Tag,
// }
