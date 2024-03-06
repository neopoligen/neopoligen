use crate::collection::*;
use crate::page::Page;
use std::collections::BTreeMap;

impl Collection {
    pub fn new_from_tags(pages: &BTreeMap<String, Page>, tags: Vec<String>) -> Collection {
        let tree: Vec<CollectionItem> = vec![];

        // let mut tree: Vec<CollectionItem> = patterns
        //     .iter()
        //     .filter_map(|pattern| folder_menu_index_finder(pages, pattern.to_vec(), vec![]))
        //     .collect();
        // tree.iter_mut()
        //     .for_each(|item| sort_by_source_path(&mut item.children));

        let mut prev_next_list = vec![];
        load_prev_next(&tree, &mut prev_next_list);

        let c = Collection {
            active_ancestors: vec![],
            active_folders: vec![],
            next_item: None,
            prev_item: None,
            prev_next_list,
            tree,
        };
        c
    }
}

fn load_prev_next(items: &Vec<CollectionItem>, dest: &mut Vec<CollectionItem>) {
    items.iter().for_each(|item| {
        if !matches![item.base_type, CollectionItemBaseType::TitleFolder] {
            let prev_next_item = item.clone();
            dest.push(prev_next_item);
        }
        load_prev_next(&item.children, dest);
    });
}
