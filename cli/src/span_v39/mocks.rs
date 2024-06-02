use crate::{site_config::SiteConfig, span_v39::*};

impl SpanV39 {
    pub fn mock1_code_shorthand_with_attrs() -> SpanV39 {
        let config = SiteConfig::mock1();
        let source = r#"``print("ping")|python|class: green``"#;
        span_v39(source, &config.spans).unwrap().1
    }
}
