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
    pub fn parse(source: &str) -> PageFilter {
        PageFilter::Status {
            exclude: false,
            value: "publishd".to_string(),
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
