use crate::site::Site;
use itertools::Itertools;
use minijinja::Value;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

// This returns a list of page IDs for all
// pages that have a given tag or tags.
//
// - Results are alphabetized
//
// - Passing a filter that starts
// with a `!`` removes any pages that
// match the value

impl Site {
    pub fn filter_page_links_alpha(&self, args: &[Value]) -> Value {
        let current_page_id = &args[0].to_string();
        // Load in two BTreeSets. The first is for includes,
        // the second is for excludes
        let patterns: Vec<(BTreeSet<String>, BTreeSet<String>)> = args
            .iter()
            .skip(1)
            .map(|arg| {
                let mut includes = BTreeSet::new();
                let mut exlcudes = BTreeSet::new();
                arg.try_iter().unwrap().for_each(|item| {
                    let the_string = String::from(item);
                    if the_string.starts_with("!") {
                        exlcudes.insert(the_string.strip_prefix("!").unwrap().to_string());
                    } else {
                        includes.insert(the_string);
                    }
                });
                (includes, exlcudes)
            })
            .collect();
        let results: Vec<_> = self
            .page_data
            .iter()
            .filter_map(|p| {
                let filters: BTreeSet<String> = p.1.filters.clone().into_iter().collect();
                patterns.iter().find_map(|pattern| {
                    // this is the exclude matcher
                    if !filters.is_disjoint(&pattern.1) {
                        None
                    // this is the includes matcher
                    } else if filters.is_superset(&pattern.0) {
                        match p.1.full_title.clone() {
                            Some(full_title) => {
                                let mut d = BTreeMap::new();
                                d.insert("id", p.0.to_string());
                                // TODO: split out the link and full titles
                                // when they are ready. Right now, it's just
                                // a stub
                                d.insert("link_title", full_title.clone());
                                d.insert("full_title", full_title.clone());
                                d.insert(
                                    "link",
                                    format!(
                                        r#"<a href="{}">{}</a>"#,
                                        p.1.url_path.clone().unwrap(),
                                        full_title.clone()
                                    ),
                                );
                                if p.0 == current_page_id {
                                    d.insert("link_or_title", full_title.clone());
                                } else {
                                    d.insert(
                                        "link_or_title",
                                        format!(
                                            r#"<a href="{}">{}</a>"#,
                                            p.1.url_path.clone().unwrap(),
                                            full_title.clone()
                                        ),
                                    );
                                }
                                Some(d)
                                // Some((p.0.to_string(), p.1.url_path.clone().unwrap(), title))
                            }
                            None => None,
                        }
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.get("link_title"), &b.get("link_title")))
            .collect();
        // dbg!(results);
        Value::from_serializable(&results)
    }
}
