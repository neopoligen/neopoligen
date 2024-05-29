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

#[derive(Debug)]
pub enum PageFilter {
    Status { exclude: bool, value: String }, // pub exclude: bool,
                                             // pub r#type: PageFilterType,
                                             // pub value: String,
}

// #[derive(Debug)]
// pub enum PageFilterType {
//     RootFolder,
//     Folder,
//     Status,
//     Tag,
// }
