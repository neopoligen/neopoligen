use crate::collection::*;
use crate::page::Page;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl Collection {
    pub fn new_from_tags(pages: &BTreeMap<String, Page>, tags: Vec<String>) -> Collection {
        let tag_set = BTreeSet::from_iter(tags.clone());
        let mut tree: Vec<CollectionItem> = pages
            .iter()
            .filter_map(|page| check_page_for_tags(page.1, &tag_set))
            .collect();
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

fn check_page_for_tags(page: &Page, tags: &BTreeSet<String>) -> Option<CollectionItem> {
    if page.tags.intersection(tags).collect::<Vec<_>>().len() > 0 {
        Some(CollectionItem {
            // TODO: Update this to Title and Folder
            base_type: CollectionItemBaseType::Page,
            ancestors: vec![],
            children: vec![],
            folders: page.folders.clone(),
            id: page.id.clone(),
            sort_source_path: page.path_parts.join(""),
            status: CollectionItemStatus::ToBeDetermined,
        })
    } else {
        None
    }
}
