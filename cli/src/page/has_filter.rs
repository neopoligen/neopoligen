use crate::page::Page;
use minijinja::Value;

impl Page {
    pub fn has_filter(&self, args: &[Value]) -> Value {
        let match_filter = &args[0].to_string().to_lowercase();
        if self.filters().contains(match_filter) {
            Value::from(true)
        } else {
            Value::from(false)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn page_has_filter_true() {
        let p = Page::test_with_filters_section();
        let arg1 = Value::from("hoTEL");
        let left = Value::from(true);
        let right = p.has_filter(&[arg1]);
        assert_eq!(left, right);
    }

    #[test]
    fn page_has_filter_false() {
        let p = Page::test_with_filters_section();
        let arg1 = Value::from("no-match");
        let left = Value::from(false);
        let right = p.has_filter(&[arg1]);
        assert_eq!(left, right);
    }
}
