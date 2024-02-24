use crate::site::Site;
use minijinja::Value;

impl Site {
    //! [x] Takes 2 ids (required)
    //!
    //! [x] Takes optional third arg that's a title
    //!
    //! [x] If the 2 ids are different, a link is created
    //!     pointing to the first id with its title
    //!     as the text
    //!
    //! [x] If the 2 ids are the same the title of the page
    //!     is returned without being made into a link
    //!
    //! [x] If the first id (aka the target id) doesn't
    //!     exist in the site's list of pages a "page unavailable"
    //!     message is returned

    pub fn link_or_title(&self, args: &[Value]) -> Value {
        if args.len() >= 2 {
            let target_id = args[0].to_string();
            let current_id = args[1].to_string();
            match self.page_data.get(&target_id) {
                Some(pd) => {
                    let title = if args.len() >= 3 {
                        args[2].to_string()
                    } else {
                        pd.full_title.clone().unwrap()
                    };

                    if target_id == current_id {
                        Value::from(title)
                    } else {
                        Value::from(format!(
                            r#"<a href="{}">{}</a>"#,
                            pd.url_path.clone().unwrap(),
                            title
                        ))
                    }
                }
                None => Value::from(r#"page unavailable"#),
            }
        } else {
            Value::from(r#"page unavailable"#)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use minijinja::value::Value;
    use pretty_assertions::assert_eq;

    #[test]
    fn link_or_title_with_title() {
        let site = Site::site_with_2_pages();
        let arg1 = Value::from("id12345c");
        let arg2 = Value::from("id12345c");
        let left = Value::from("Delta Oscar");
        let right = site.link_or_title(&[arg1, arg2]);
        assert_eq!(left, right);
    }

    #[test]
    fn link_or_title_with_link() {
        let site = Site::site_with_2_pages();
        let arg1 = Value::from("id003333");
        let arg2 = Value::from("id12345c");
        let left = Value::from(r#"<a href="/en/id003333/?victor-papa">Victor Papa</a>"#);
        let right = site.link_or_title(&[arg1, arg2]);
        assert_eq!(left, right);
    }

    #[test]
    fn link_or_title_missing_target_page() {
        let site = Site::site_with_2_pages();
        let arg1 = Value::from("invalid_id");
        let arg2 = Value::from("id12345c");
        let left = Value::from(r#"page unavailable"#);
        let right = site.link_or_title(&[arg1, arg2]);
        assert_eq!(left, right);
    }
}
