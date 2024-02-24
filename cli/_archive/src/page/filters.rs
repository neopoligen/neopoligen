use crate::child::Child;
use crate::page::Page;

// Creates a list of strings that
// includes the tags as well as);
// the type and status for the page
// designed to make it easier to filter

impl Page {
    pub fn filters(&self) -> Vec<String> {
        let mut output = match self.ast.iter().find_map(|child| {
            if let Child::Section(section) = child {
                let section_type = &section.r#type;
                if section_type == "filters" {
                    Some(
                        section
                            .flag_attributes
                            .iter()
                            .map(|flag| flag.to_lowercase().to_string())
                            .collect::<Vec<String>>(),
                    )
                } else {
                    None
                }
            } else {
                None
            }
        }) {
            Some(f) => f,
            None => vec![],
        };
        match self.r#type() {
            Some(t) => output.push(t.to_lowercase()),
            None => (),
        }
        match self.status() {
            Some(s) => output.push(s.to_lowercase()),
            None => (),
        }
        match self.id() {
            Some(id) => output.push(id.to_lowercase()),
            None => (),
        }
        output.append(&mut self.folders());
        output.append(&mut self.tags());
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    // NOTE: The 'hotel' checks for case-insensitivity
    // (the source file has 'HOTEL' in call caps

    #[test]
    fn filter_basic_test() {
        let p = Page::test_with_filters_section();
        let left = vec![
            "lima".to_string(),
            "filter-test".to_string(),
            "published".to_string(),
            "id778866".to_string(),
            "hotel".to_string(),
        ];
        let right = p.filters();
        assert_eq!(left, right);
    }
}
