pub mod mocks;

use minijinja::Value;

#[derive(Debug, PartialEq)]
pub struct PageFilterOrSet {
    pub and_groups: Vec<PageFilterAndGroup>,
}

impl PageFilterOrSet {
    // DEPRECATED: TODO: remove this in favor of .parse()
    pub fn new() -> PageFilterOrSet {
        PageFilterOrSet { and_groups: vec![] }
    }

    pub fn parse(args: &[Value]) -> Option<PageFilterOrSet> {
        // TODO: Error handling
        let and_groups = args
            .iter()
            .filter_map(|ag| {
                if let Ok(and_iter) = ag.try_iter() {
                    Some(PageFilterAndGroup {
                        filters: and_iter
                            .into_iter()
                            .filter_map(|filter_string| {
                                if let Some(text) = filter_string.as_str() {
                                    PageFilter::parse(text)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<PageFilter>>(),
                    })
                } else {
                    None
                }
            })
            .collect();
        Some(PageFilterOrSet { and_groups })
    }
}

#[derive(Debug, PartialEq)]
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
