// use crate::site_v2::SiteV2;
// use minijinja::value::ValueKind;
// use minijinja::Value;
// use std::collections::BTreeSet;

// NOTE: This is only for filtering out, there's
// no function to add a `!`` for overriding an
// exclude with an include. That would be adding
// an include after an exclude which feels
// like a source of logic problems and goes against
// the general:
// implicit exclude, explicit include, explicit exclude
// since there is no "implicit exclude" in this method.
// That is, everything is included by default

// impl SiteV2 {
//     pub fn link_or_filtered_title(&self, args: &[Value]) -> Option<String> {
//         if args.len() >= 3 {
//             let current_id = args[0].to_string();
//             if let Some(current_page) = self.pages.get(&current_id) {
//                 let target_id = args[1].to_string();
//                 if let Some(target_page) = self.pages.get(&target_id) {
//                     let title = match args[3].get_attr("title") {
//                         Ok(result) => {
//                             if result.is_undefined() {
//                                 target_page.full_title().clone().unwrap()
//                             } else {
//                                 result.to_string()
//                             }
//                         }
//                         Err(_) => "no title available".to_string(),
//                     };
//                     if target_id == current_id {
//                         Some(title)
//                     } else {
//                         let page_filters: BTreeSet<String> =
//                             current_page.filters().clone().into_iter().collect();
//                         let mut exclude_tags = BTreeSet::new();
//                         let _ = &args[2].try_iter().unwrap().for_each(|arg| {
//                             let tag = String::from(arg);
//                             exclude_tags.insert(tag);
//                         });
//                         if exclude_tags.is_empty() {
//                             Some(format!(
//                                 r#"<a href="{}">{}</a>"#,
//                                 target_page.href().clone().unwrap(),
//                                 title
//                             ))
//                         } else if page_filters.is_disjoint(&exclude_tags) {
//                             Some(format!(
//                                 r#"<a href="{}">{}</a>"#,
//                                 target_page.href().clone().unwrap(),
//                                 title
//                             ))
//                         } else {
//                             Some(title)
//                         }
//                     }
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use pretty_assertions::assert_eq;
//     use std::collections::BTreeMap;
//     #[test]
//     fn link_or_filtered_title_same_page_returns_title() {
//         let site = SiteV2::site_with_eight_pages();
//         let arg1 = Value::from("id12345c");
//         let arg2 = Value::from("id12345c");
//         let arg3 = Value::from_serializable::<Vec<String>>(&vec![]);
//         let arg4_data: BTreeMap<String, String> = BTreeMap::new();
//         let arg4 = Value::from_serializable(&arg4_data);
//         let left = Some(r#"Delta Oscar"#.to_string());
//         let right = site.link_or_filtered_title(&[arg1, arg2, arg3, arg4]);
//         assert_eq!(left, right);
//     }
//     #[test]
//     fn link_or_filtered_title_target_without_filters() {
//         let site = SiteV2::site_with_eight_pages();
//         let arg1 = Value::from("id12345c");
//         let arg2 = Value::from("id12345e");
//         let arg3 = Value::from_serializable::<Vec<Vec<String>>>(&vec![]);
//         let arg4_data: BTreeMap<String, String> = BTreeMap::new();
//         let arg4 = Value::from_serializable(&arg4_data);
//         let left = Some(r#"<a href="/en/id12345e/?tango-foxtrot">Tango Foxtrot</a>"#.to_string());
//         let right = site.link_or_filtered_title(&[arg1, arg2, arg3, arg4]);
//         assert_eq!(left, right);
//     }
//     #[test]
//     fn link_or_filtered_title_target_with_non_matching_filters() {
//         let site = SiteV2::site_with_eight_pages();
//         let arg1 = Value::from("id12345c");
//         let arg2 = Value::from("id003333");
//         let arg3 = Value::from_serializable::<Vec<Vec<String>>>(&vec![vec![
//             "non-matching-filter-here".to_string(),
//         ]]);
//         let arg4_data: BTreeMap<String, String> = BTreeMap::new();
//         let arg4 = Value::from_serializable(&arg4_data);
//         let left = Some(r#"<a href="/en/id003333/?victor-papa">Victor Papa</a>"#.to_string());
//         let right = site.link_or_filtered_title(&[arg1, arg2, arg3, arg4]);
//         assert_eq!(left, right);
//     }
//     #[test]
//     fn link_or_filtered_title_target_with_matching_include_filter() {
//         let site = SiteV2::site_with_eight_pages();
//         let arg1 = Value::from("id12345c");
//         let arg2 = Value::from("id12345d");
//         let arg3 = Value::from_serializable::<Vec<String>>(&vec!["main-body-test".to_string()]);
//         let arg4_data: BTreeMap<String, String> = BTreeMap::new();
//         let arg4 = Value::from_serializable(&arg4_data);
//         let left = Some(r#"Bravo Charlie"#.to_string());
//         let right = site.link_or_filtered_title(&[arg1, arg2, arg3, arg4]);
//         assert_eq!(left, right);
//     }
//     #[test]
//     fn link_or_filtered_title_target_with_override_title() {
//         let site = SiteV2::site_with_eight_pages();
//         let arg1 = Value::from("id12345c");
//         let arg2 = Value::from("id12345d");
//         let arg3 = Value::from_serializable::<Vec<String>>(&vec!["main-body-test".to_string()]);
//         let mut arg4_data: BTreeMap<String, String> = BTreeMap::new();
//         arg4_data.insert("title".to_string(), "This Is An Override Title".to_string());
//         let arg4 = Value::from_serializable(&arg4_data);
//         let left = Some(r#"This Is An Override Title"#.to_string());
//         let right = site.link_or_filtered_title(&[arg1, arg2, arg3, arg4]);
//         assert_eq!(left, right);
//     }
// }
