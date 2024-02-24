// DEPRECATED: Remove when site_v2 is working
//
use crate::site::Site;
use minijinja::Value;
use std::collections::BTreeSet;

// NOTE, this is only for include patterns
// right now. TODO: [] Add `!`` exclude
// patterns

impl Site {
    pub fn link_or_title_filtered(&self, args: &[Value]) -> Value {
        if args.len() >= 3 {
            let current_id = args[1].to_string();
            if let Some(current_pd) = self.page_data.get(&current_id) {
                let target_id = args[0].to_string();
                if let Some(target_pd) = self.page_data.get(&target_id) {
                    // let current_id = args[1].to_string();
                    // let target_id = args[0].to_string();
                    // let target_pd = self.page_data.get(&target_id).unwrap();

                    let title = if args.len() == 4 {
                        args[3].to_string()
                    } else {
                        target_pd.full_title.clone().unwrap()
                    };
                    if target_id == current_id {
                        Value::from(title)
                    } else {
                        // Load in two BTreeSets. The first is for includes,
                        // the second is for excludes
                        let mut includes = BTreeSet::new();
                        let mut excludes = BTreeSet::new();
                        args[2].try_iter().unwrap().for_each(|item| {
                            let the_string = String::from(item);
                            if the_string.starts_with("!") {
                                excludes.insert(the_string.strip_prefix("!").unwrap().to_string());
                            } else {
                                includes.insert(the_string);
                            }
                        });

                        let filters: BTreeSet<String> =
                            current_pd.filters.clone().into_iter().collect();
                        if includes.is_subset(&filters) {
                            Value::from(title)
                        } else {
                            Value::from(format!(
                                r#"<a href="{}">{}</a>"#,
                                target_pd.url_path.clone().unwrap(),
                                title
                            ))
                        }
                    }
                } else {
                    Value::from(())
                }
            } else {
                Value::from(())
            }
        } else {
            Value::from(())
        }
    }
}
