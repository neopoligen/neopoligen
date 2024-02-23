use crate::child::Child;
use crate::page::Page;
use std::collections::BTreeMap;

impl Page {
    pub fn metadata(&self) -> BTreeMap<String, String> {
        let mut md: BTreeMap<String, String> = BTreeMap::new();
        self.ast.iter().for_each(|child| {
            if let Child::Section(section) = child {
                if &section.r#type == "metadata" {
                    section.key_value_attributes.iter().for_each(|kv| {
                        md.insert(kv.0.to_string(), kv.1.to_string());
                    })
                }
            }
        });
        md
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn metadata_for_page_exists() {
        let p = Page::id12345d_title_and_metadata();
        let mut left = BTreeMap::new();
        left.insert("date".to_string(), "2024-01-02 03:04:05".to_string());
        left.insert("id".to_string(), "id12345d".to_string());
        left.insert("type".to_string(), "builder-test".to_string());
        left.insert("status".to_string(), "scratch".to_string());
        left.insert("extra_metadata".to_string(), "testing value".to_string());
        let right = p.metadata();
        assert_eq!(left, right);
    }
}
