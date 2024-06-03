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
                                    PageFilterV2::parse(text)
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<PageFilterV2>>(),
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
    pub filters: Vec<PageFilterV2>,
}

#[derive(Debug, PartialEq)]
pub enum PageFilter {
    Status { exclude: bool, value: String },
    Type { exclude: bool, value: String },
}

#[derive(Debug, PartialEq)]
pub struct PageFilterV2 {
    pub exclude_request: bool,
    pub kind: PageFilterV2Kind,
}

#[derive(Debug, PartialEq)]
pub enum PageFilterV2Kind {
    Status { value_to_match: String },
    Type { value_to_match: String },
}

impl PageFilterV2 {
    pub fn parse(source: &str) -> Option<PageFilterV2> {
        if let Some(parts) = source.split_once(":") {
            let (exclude, value) = match parts.1.strip_prefix("!") {
                Some(value) => (true, value.trim().to_string()),
                None => (false, parts.1.trim().to_string()),
            };
            match parts.0 {
                "status" => Some(PageFilterV2 {
                    exclude_request: exclude,
                    kind: PageFilterV2Kind::Status {
                        value_to_match: value,
                    },
                }),
                "type" => Some(PageFilterV2 {
                    exclude_request: exclude,
                    kind: PageFilterV2Kind::Type {
                        value_to_match: value,
                    },
                }),
                _ => None,
            }
        } else {
            None
        }
    }
}

// // DEPRECATED: Move to PageFilterV2
// impl PageFilter {
//     pub fn parse(source: &str) -> Option<PageFilter> {
//         if let Some(parts) = source.split_once(":") {
//             let (exclude, value) = match parts.1.strip_prefix("!") {
//                 Some(value) => (true, value.trim().to_string()),
//                 None => (false, parts.1.trim().to_string()),
//             };
//             match parts.0 {
//                 "status" => Some(PageFilter::Status { exclude, value }),
//                 "type" => Some(PageFilter::Type { exclude, value }),
//                 _ => None,
//             }
//         } else {
//             None
//         }
//     }
// }

// #[derive(Debug)]
// pub enum PageFilterType {
//     RootFolder,
//     Folder,
//     Status,
//     Tag,
// }
