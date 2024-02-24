use crate::page::Page;
use std::path::PathBuf;

impl Page {
    pub fn r#type(&self) -> Option<String> {
        if let Some(t) = self.metadata().get("type") {
            return Some(t.to_string());
        } else if self.folders().len() > 0 {
            self.folders().iter().nth(0).cloned()
        } else {
            return Some("post".to_string());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn page_type_from_metadata() {
        let p = Page::id12345d_title_and_metadata();
        let left = Some("builder-test".to_string());
        let right = p.r#type();
        assert_eq!(left, right);
    }

    #[test]
    fn page_type_from_folder() {
        let p = Page::id441122_type_from_folder();
        let left = Some("example_type_folder".to_string());
        let right = p.r#type();
        assert_eq!(left, right);
    }

    #[test]
    fn page_type_no_type_or_folder() {
        let p = Page::no_type_or_folder();
        let left = Some("post".to_string());
        let right = p.r#type();
        assert_eq!(left, right);
    }
}
